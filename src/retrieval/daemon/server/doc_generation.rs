//! Background documentation generation
//!
//! Runs doc generation in a background thread using
//! parallel code extraction and serial LLM inference.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use super::doc_gen_batch::{extract_batch, run_llm_batch};
use super::types::{
	GraphAndSymbols, SharedDocGenerator, SharedDocStores,
};
use super::DocGenProgress;
use crate::retrieval::docgen::{DocGenerator, DocLinker};

/// Shared state for background doc generation
pub(crate) struct DocGenContext {
	/// doc stores shared with daemon
	pub stores: SharedDocStores,
	/// shared doc generator (lazy-loaded)
	pub generator: SharedDocGenerator,
	/// progress tracking
	pub progress: Arc<Mutex<DocGenProgress>>,
	/// cancellation flag
	pub cancel: Arc<AtomicBool>,
}

/// Handle cancellation: mark done and cleanup
#[allow(clippy::print_stderr)]
fn handle_cancellation(
	progress: &Arc<Mutex<DocGenProgress>>,
	canonical: &Path,
) {
	let Ok(mut prog) = progress.lock() else {
		return;
	};
	prog.is_running = false;
	let docs = canonical.join(".rustean-index/docs.json");
	let _ = std::fs::remove_file(&docs);
	eprintln!(
		"[daemon] Cancelled: removed partial docs.json"
	);
}

/// Run doc generation in background thread
///
/// Hybrid pipeline: templates first (instant, no GPU),
/// then LLM only for complex symbols.
/// Model is unloaded after completion to free GPU.
#[allow(clippy::print_stderr)]
pub(crate) fn run_doc_gen_background(
	ctx: DocGenContext,
	canonical: PathBuf,
	pending_ids: Vec<String>,
	graph_and_symbols: GraphAndSymbols,
) {
	let llm_ids =
		run_template_phase(&ctx, &canonical, &pending_ids);

	if llm_ids.is_empty() {
		complete_doc_gen(
			&ctx, &canonical, 0, graph_and_symbols,
		);
		return;
	}
	run_llm_phase(&ctx, &canonical, &llm_ids, graph_and_symbols);
}

/// Phase 1: partition and process templates instantly
#[allow(clippy::print_stderr)]
fn run_template_phase(
	ctx: &DocGenContext,
	canonical: &PathBuf,
	pending_ids: &[String],
) -> Vec<String> {
	let (tpl_ids, llm_ids) =
		super::doc_gen_template::partition_pending(
			&ctx.stores, canonical, pending_ids,
		);
	eprintln!(
		"[daemon] Split: {} template, {} LLM",
		tpl_ids.len(), llm_ids.len(),
	);
	super::doc_gen_template::process_template_batch(
		&ctx.stores, canonical,
		&tpl_ids, &ctx.progress,
	);
	llm_ids
}

/// Phase 2: LLM inference for complex symbols
fn run_llm_phase(
	ctx: &DocGenContext,
	canonical: &PathBuf,
	llm_ids: &[String],
	graph_and_symbols: GraphAndSymbols,
) {
	if !init_generator(&ctx.generator, &ctx.progress) {
		return;
	}
	let result =
		process_pending_entries(ctx, canonical, llm_ids);
	if result.cancelled {
		handle_cancellation(&ctx.progress, canonical);
	} else {
		complete_doc_gen(
			ctx, canonical,
			result.generated, graph_and_symbols,
		);
	}
	super::doc_gen_helpers::unload_generator(
		&ctx.generator,
	);
}

/// Log completion and finalize docs after generation
#[allow(clippy::print_stderr)]
fn complete_doc_gen(
	ctx: &DocGenContext,
	canonical: &PathBuf,
	generated: usize,
	graph_and_symbols: GraphAndSymbols,
) {
	eprintln!(
		"[daemon] Generated {} docs for {}",
		generated, canonical.display()
	);
	finalize_docs(
		&ctx.stores, &ctx.progress,
		canonical, graph_and_symbols,
	);
}

/// Initialize the doc generator if not yet loaded
///
/// Returns true if ready, false on failure.
#[allow(clippy::print_stderr)]
fn init_generator(
	generator: &SharedDocGenerator,
	progress: &Arc<Mutex<DocGenProgress>>,
) -> bool {
	let Ok(mut gen_lock) = generator.lock() else {
		return false;
	};
	if gen_lock.is_some() {
		return true;
	}
	eprintln!("[daemon] Initializing doc generator...");
	match DocGenerator::with_default_model() {
		Ok(doc_gen) => {
			eprintln!("[daemon] Doc generator ready");
			*gen_lock = Some(doc_gen);
			true
		}
		Err(err) => {
			eprintln!(
				"[daemon] Failed to load doc generator: {}",
				err
			);
			if let Ok(mut prog) = progress.lock() {
				prog.is_running = false;
			}
			false
		}
	}
}

/// Result of processing pending entries
struct ProcessResult {
	/// total docs successfully generated
	generated: usize,
	/// whether processing was cancelled
	cancelled: bool,
}

/// Process a single batch chunk through extraction + LLM
#[allow(clippy::too_many_arguments)]
fn process_chunk(
	ctx: &DocGenContext,
	canonical: &PathBuf,
	chunk: &[String],
	save_interval: usize,
	total_processed: &mut usize,
	generated: &mut usize,
) -> bool {
	let prepared =
		extract_batch(&ctx.stores, canonical, chunk);
	run_llm_batch(
		prepared,
		&ctx.stores,
		&ctx.generator,
		&ctx.progress,
		&ctx.cancel,
		canonical,
		save_interval,
		total_processed,
		generated,
	)
}

/// Process pending doc entries in batches
///
/// Parallel code extraction + serial LLM inference.
#[allow(clippy::print_stderr)]
fn process_pending_entries(
	ctx: &DocGenContext,
	canonical: &PathBuf,
	pending_ids: &[String],
) -> ProcessResult {
	let mut generated = 0;
	let save_interval = 10;
	let mut total_processed = 0;

	for chunk in pending_ids.chunks(20) {
		if ctx.cancel.load(Ordering::Relaxed) {
			eprintln!("[daemon] Doc gen cancelled");
			return ProcessResult {
				generated,
				cancelled: true,
			};
		}
		if process_chunk(
			ctx, canonical, chunk, save_interval,
			&mut total_processed, &mut generated,
		) {
			return ProcessResult {
				generated,
				cancelled: true,
			};
		}
	}
	ProcessResult {
		generated,
		cancelled: false,
	}
}

/// Build cross-references and do final save
fn finalize_docs(
	stores: &SharedDocStores,
	progress: &Arc<Mutex<DocGenProgress>>,
	canonical: &PathBuf,
	graph_and_symbols: GraphAndSymbols,
) {
	build_cross_refs(stores, canonical, graph_and_symbols);
	save_doc_store(stores, canonical);
	mark_generation_done(progress);
}

/// Build cross-reference links if graph is available
fn build_cross_refs(
	stores: &SharedDocStores,
	canonical: &PathBuf,
	graph_and_symbols: GraphAndSymbols,
) {
	let Some((graph, syms)) = graph_and_symbols
	else {
		return;
	};
	let Ok(mut lock) = stores.lock() else {
		return;
	};
	let Some(store) = lock.get_mut(canonical) else {
		return;
	};
	let linker = DocLinker::new();
	linker.build_links(store, &graph, &syms);
}

/// Save doc store to disk
#[allow(clippy::print_stderr)]
fn save_doc_store(
	stores: &SharedDocStores,
	canonical: &PathBuf,
) {
	let Ok(lock) = stores.lock() else { return };
	let Some(store) = lock.get(canonical) else {
		return;
	};
	if let Err(err) = store.save() {
		eprintln!(
			"[daemon] Failed to save: {}", err
		);
	}
}

/// Mark doc generation as no longer running
fn mark_generation_done(
	progress: &Arc<Mutex<DocGenProgress>>,
) {
	if let Ok(mut prog) = progress.lock() {
		prog.is_running = false;
	}
}
