//! Background documentation generation
//!
//! Runs doc generation in a background thread using
//! parallel code extraction and serial LLM inference.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use super::doc_gen_batch::{extract_batch, run_llm_batch};
use super::DocGenProgress;
use crate::retrieval::docgen::{
	DocGenerator, DocLinker, DocStore,
};

/// Run doc generation in background thread
///
/// Locks are held briefly: extract entry -> unlock ->
/// LLM inference -> lock -> write back. This lets the
/// main thread serve status/query requests.
pub(crate) fn run_doc_gen_background(
	stores: Arc<Mutex<HashMap<PathBuf, DocStore>>>,
	generator: Arc<Mutex<Option<DocGenerator>>>,
	progress: Arc<Mutex<DocGenProgress>>,
	cancel: Arc<AtomicBool>,
	canonical: PathBuf,
	pending_ids: Vec<String>,
	graph_and_symbols: Option<(
		crate::indexer::SemanticGraph,
		Vec<crate::indexer::Symbol>,
	)>,
) {
	// initialize doc generator if needed (~30s)
	if !init_generator(&generator, &progress) {
		return;
	}

	let result = process_pending_entries(
		&stores,
		&generator,
		&progress,
		&cancel,
		&canonical,
		&pending_ids,
	);

	// on cancel: mark done and delete partial docs
	if result.cancelled {
		let mut prog = progress.lock().unwrap();
		prog.is_running = false;
		let docs = canonical.join(".ch-index/docs.json");
		let _ = std::fs::remove_file(&docs);
		eprintln!(
			"[daemon] Cancelled: removed partial docs.json"
		);
		return;
	}

	eprintln!(
		"[daemon] Generated {} docs for {}",
		result.generated,
		canonical.display()
	);

	finalize_docs(
		&stores,
		&progress,
		&canonical,
		graph_and_symbols,
	);
}

/// Initialize the doc generator if not yet loaded
///
/// Returns true if ready, false on failure.
fn init_generator(
	generator: &Arc<Mutex<Option<DocGenerator>>>,
	progress: &Arc<Mutex<DocGenProgress>>,
) -> bool {
	let mut gen_lock = generator.lock().unwrap();
	if gen_lock.is_some() {
		return true;
	}

	eprintln!("[daemon] Initializing doc generator...");
	match DocGenerator::with_default_model() {
		Ok(gen) => {
			eprintln!("[daemon] Doc generator ready");
			*gen_lock = Some(gen);
			true
		}
		Err(e) => {
			eprintln!(
				"[daemon] Failed to load doc generator: {}",
				e
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

/// Process pending doc entries in batches
///
/// Parallel code extraction + serial LLM inference.
fn process_pending_entries(
	stores: &Arc<Mutex<HashMap<PathBuf, DocStore>>>,
	generator: &Arc<Mutex<Option<DocGenerator>>>,
	progress: &Arc<Mutex<DocGenProgress>>,
	cancel: &Arc<AtomicBool>,
	canonical: &PathBuf,
	pending_ids: &[String],
) -> ProcessResult {
	let mut generated = 0; // success count
	let save_interval = 10; // save every N entries
	let batch_size = 20; // parallel batch size
	let mut cancelled = false;
	let mut total_processed = 0;

	for chunk in pending_ids.chunks(batch_size) {
		if cancel.load(Ordering::Relaxed) {
			eprintln!("[daemon] Doc gen cancelled");
			cancelled = true;
			break;
		}

		let prepared =
			extract_batch(stores, canonical, chunk);

		let batch_result = run_llm_batch(
			&prepared,
			stores,
			generator,
			progress,
			cancel,
			canonical,
			save_interval,
			&mut total_processed,
			&mut generated,
		);

		if batch_result {
			cancelled = true;
			break;
		}
	}

	ProcessResult { generated, cancelled }
}

/// Build cross-references and do final save
fn finalize_docs(
	stores: &Arc<Mutex<HashMap<PathBuf, DocStore>>>,
	progress: &Arc<Mutex<DocGenProgress>>,
	canonical: &PathBuf,
	graph_and_symbols: Option<(
		crate::indexer::SemanticGraph,
		Vec<crate::indexer::Symbol>,
	)>,
) {
	// build cross-references if we have graph
	if let Some((graph, syms)) = graph_and_symbols {
		let mut lock = stores.lock().unwrap();
		if let Some(store) = lock.get_mut(canonical) {
			let linker = DocLinker::new();
			linker.build_links(store, &graph, &syms);
		}
	}

	// final save
	{
		let lock = stores.lock().unwrap();
		if let Some(store) = lock.get(canonical) {
			if let Err(e) = store.save() {
				eprintln!(
					"[daemon] Failed to save doc store: {}",
					e
				);
			}
		}
	}

	// mark as done
	{
		let mut prog = progress.lock().unwrap();
		prog.is_running = false;
	}
}
