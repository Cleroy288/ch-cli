//! Batch processing for doc generation
//!
//! Parallel code extraction and serial LLM inference
//! for documentation entries.

use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use rayon::prelude::*;

use super::DocGenProgress;
use crate::retrieval::docgen::generator_utils::extract_code_snippet;
use crate::retrieval::docgen::prompts::build_prompt;
use crate::retrieval::docgen::{DocGenerator, DocStore};

/// Extract code snippets + build prompts in parallel
pub(crate) fn extract_batch(
	stores: &Arc<Mutex<HashMap<PathBuf, DocStore>>>,
	canonical: &PathBuf,
	chunk: &[String],
) -> Vec<(
	String,
	crate::retrieval::docgen::DocEntry,
	String,
)> {
	// extract entries from store (brief lock)
	let entries: Vec<(
		String,
		crate::retrieval::docgen::DocEntry,
	)> = {
		let lock = stores.lock().unwrap();
		let store = match lock.get(canonical) {
			Some(s) => s,
			None => return Vec::new(),
		};
		chunk
			.iter()
			.filter_map(|id| {
				store
					.get(id)
					.map(|e| (id.clone(), e.clone()))
			})
			.collect()
	};

	// parallel: extract code + build prompts
	entries
		.into_par_iter()
		.filter_map(|(id, mut entry)| {
			if entry.code_snippet.is_empty() {
				if let Ok(snippet) =
					extract_code_snippet(
						&entry.file_path,
						entry.line,
					)
				{
					entry.code_snippet = snippet;
				}
			}
			let prompt = build_prompt(
				entry.kind,
				&entry.name,
				entry.signature.as_deref(),
				&entry.code_snippet,
				entry.user_comment.as_deref(),
				entry.links.parent.as_deref(),
			);
			Some((id, entry, prompt))
		})
		.collect()
}

/// Run LLM inference serially on prepared entries
///
/// Returns true if cancelled during this batch.
#[allow(clippy::too_many_arguments)]
pub(crate) fn run_llm_batch(
	prepared: &[(
		String,
		crate::retrieval::docgen::DocEntry,
		String,
	)],
	stores: &Arc<Mutex<HashMap<PathBuf, DocStore>>>,
	generator: &Arc<Mutex<Option<DocGenerator>>>,
	progress: &Arc<Mutex<DocGenProgress>>,
	cancel: &Arc<AtomicBool>,
	canonical: &PathBuf,
	save_interval: usize,
	total_processed: &mut usize,
	generated: &mut usize,
) -> bool {
	for (_id, entry, prompt) in prepared {
		if cancel.load(Ordering::Relaxed) {
			eprintln!("[daemon] Doc gen cancelled");
			return true;
		}

		let mut entry = entry.clone();
		let success = {
			let mut gen = generator.lock().unwrap();
			if let Some(ref mut g) = *gen {
				match g.generate_from_prompt(
					&mut entry, prompt,
				) {
					Ok(()) => true,
					Err(e) => {
						eprintln!(
							"[daemon] Doc gen error \
							 for {}: {}",
							entry.name, e
						);
						false
					}
				}
			} else {
				return true; // generator gone
			}
		};

		// write result back (brief lock)
		{
			let mut lock = stores.lock().unwrap();
			if let Some(store) =
				lock.get_mut(canonical)
			{
				store.upsert(entry);
			}
		}

		// update progress
		if success {
			*generated += 1;
		}
		*total_processed += 1;
		{
			let mut prog = progress.lock().unwrap();
			prog.completed += 1;
			if !success {
				prog.failed += 1;
			}
		}

		// save periodically
		if *total_processed % save_interval == 0 {
			let lock = stores.lock().unwrap();
			if let Some(store) = lock.get(canonical) {
				if let Err(e) = store.save() {
					eprintln!(
						"[daemon] Failed to save: {}",
						e
					);
				}
			}
		}
	}
	false
}
