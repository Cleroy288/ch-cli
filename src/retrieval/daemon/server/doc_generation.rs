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
	let mut prog = progress.lock().unwrap();
	prog.is_running = false;
	let docs = canonical.join(".ch-index/docs.json");
	let _ = std::fs::remove_file(&docs);
	eprintln!(
		"[daemon] Cancelled: removed partial docs.json"
	);
}

/// Run doc generation in background thread
///
/// Locks are held briefly: extract entry -> unlock ->
/// LLM inference -> lock -> write back.
/// Model is unloaded after completion to free GPU.
#[allow(clippy::print_stderr)]
pub(crate) fn run_doc_gen_background(
	ctx: DocGenContext,
	canonical: PathBuf,
	pending_ids: Vec<String>,
	graph_and_symbols: GraphAndSymbols,
) {
	if !init_generator(&ctx.generator, &ctx.progress) {
		return;
	}
	let result = process_pending_entries(
		&ctx,
		&canonical,
		&pending_ids,
	);
	if result.cancelled {
		handle_cancellation(&ctx.progress, &canonical);
	} else {
		eprintln!(
			"[daemon] Generated {} docs for {}",
			result.generated,
			canonical.display()
		);
		finalize_docs(
			&ctx.stores,
			&ctx.progress,
			&canonical,
			graph_and_symbols,
		);
	}
	// Free ~500MB GPU: unload model after doc gen
	super::doc_gen_helpers::unload_generator(
		&ctx.generator,
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
	let mut gen_lock = generator.lock().unwrap();
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
			let mut prog = progress.lock().unwrap();
			prog.is_running = false;
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
#[allow(clippy::print_stderr)]
fn finalize_docs(
	stores: &SharedDocStores,
	progress: &Arc<Mutex<DocGenProgress>>,
	canonical: &PathBuf,
	graph_and_symbols: GraphAndSymbols,
) {
	if let Some((graph, syms)) = graph_and_symbols {
		let mut lock = stores.lock().unwrap();
		if let Some(store) = lock.get_mut(canonical) {
			let linker = DocLinker::new();
			linker.build_links(store, &graph, &syms);
		}
	}
	{
		let lock = stores.lock().unwrap();
		if let Some(store) = lock.get(canonical) {
			if let Err(err) = store.save() {
				eprintln!(
					"[daemon] Failed to save: {}",
					err
				);
			}
		}
	}
	let mut prog = progress.lock().unwrap();
	prog.is_running = false;
}
