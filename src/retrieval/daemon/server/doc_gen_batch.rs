//! Batch processing for doc generation
//!
//! Parallel code extraction and serial LLM inference
//! for documentation entries.

use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

use super::doc_gen_helpers::{
	generate_single, periodic_save,
	update_progress, write_entry_back,
};
use super::types::{
	PreparedEntry, SharedDocGenerator,
	SharedDocStores,
};
use super::DocGenProgress;
use crate::retrieval::docgen::generator_utils::extract_code_snippet;
use crate::retrieval::docgen::prompts::{
	build_prompt, PromptInput,
};
use crate::retrieval::docgen::DocEntry;

/// Read entries from store for a chunk of IDs
fn read_entries_from_store(
	stores: &SharedDocStores,
	canonical: &PathBuf,
	chunk: &[String],
) -> Vec<(String, DocEntry)> {
	let lock = stores.lock().unwrap();
	let Some(store) = lock.get(canonical) else {
		return Vec::new();
	};
	chunk
		.iter()
		.filter_map(|entry_id| {
			store
				.get(entry_id)
				.map(|entry| {
					(entry_id.clone(), entry.clone())
				})
		})
		.collect()
}

/// Build prompt for a single doc entry
fn build_entry_prompt(
	entry: &DocEntry,
) -> String {
	build_prompt(PromptInput {
		kind: entry.kind,
		name: &entry.name,
		signature: entry.signature.as_deref(),
		code_snippet: &entry.code_snippet,
		user_comment: entry.user_comment.as_deref(),
		parent: entry.links.parent.as_deref(),
	})
}

/// Extract code snippets + build prompts in parallel
pub(crate) fn extract_batch(
	stores: &SharedDocStores,
	canonical: &PathBuf,
	chunk: &[String],
) -> Vec<PreparedEntry> {
	let entries =
		read_entries_from_store(stores, canonical, chunk);
	entries
		.into_par_iter()
		.filter_map(|(entry_id, mut entry)| {
			fill_code_snippet(&mut entry);
			let prompt = build_entry_prompt(&entry);
			Some((entry_id, entry, prompt))
		})
		.collect()
}

/// Fill in code snippet if empty
fn fill_code_snippet(entry: &mut DocEntry) {
	if !entry.code_snippet.is_empty() {
		return;
	}
	if let Ok(snippet) = extract_code_snippet(
		&entry.file_path,
		entry.line,
	) {
		entry.code_snippet = snippet;
	}
}

/// Run LLM inference serially on prepared entries
///
/// Returns true if cancelled during this batch.
#[allow(clippy::too_many_arguments)]
#[allow(clippy::print_stderr)]
pub(crate) fn run_llm_batch(
	prepared: Vec<PreparedEntry>,
	stores: &SharedDocStores,
	generator: &SharedDocGenerator,
	progress: &Arc<Mutex<DocGenProgress>>,
	cancel: &Arc<AtomicBool>,
	canonical: &PathBuf,
	save_interval: usize,
	total_processed: &mut usize,
	generated: &mut usize,
) -> bool {
	for (_entry_id, mut entry, prompt) in prepared {
		if cancel.load(Ordering::Relaxed) {
			eprintln!("[daemon] Doc gen cancelled");
			return true;
		}

		let Some(success) = generate_single(
			generator, &mut entry, &prompt,
		) else {
			return true;
		};

		write_entry_back(stores, canonical, entry);

		if success {
			*generated += 1;
		}
		*total_processed += 1;
		update_progress(progress, success);

		if (*total_processed)
			.is_multiple_of(save_interval)
		{
			periodic_save(stores, canonical);
		}
	}
	false
}
