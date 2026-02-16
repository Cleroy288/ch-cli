//! Template-based batch doc generation for daemon.
//!
//! Partitions pending entries into template-eligible
//! and LLM-required groups, then processes templates
//! instantly without loading the GPU model.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use super::types::SharedDocStores;
use super::DocGenProgress;
use crate::retrieval::docgen::template_classify::{
	classify_entry, DocStrategy,
};
use crate::retrieval::docgen::template_composites::{
	generate_composite_doc,
};
use crate::retrieval::docgen::template_generate::{
	generate_template_doc,
};

/// Partition pending IDs into template and LLM groups.
///
/// Reads entries from store, classifies each, returns
/// two vectors: (template_ids, llm_ids).
#[allow(clippy::type_complexity)]
pub(crate) fn partition_pending(
	stores: &SharedDocStores,
	canonical: &PathBuf,
	ids: &[String],
) -> (Vec<String>, Vec<String>) {
	let mut tpl_ids = Vec::new();
	let mut llm_ids = Vec::new();

	let Ok(lock) = stores.lock() else {
		return (tpl_ids, llm_ids);
	};
	let Some(store) = lock.get(canonical) else {
		return (tpl_ids, llm_ids);
	};

	for entry_id in ids {
		let Some(entry) = store.get(entry_id) else {
			continue;
		};
		match classify_entry(entry) {
			DocStrategy::UserComment
			| DocStrategy::Template
			| DocStrategy::Skip => {
				tpl_ids.push(entry_id.clone());
			}
			DocStrategy::Llm => {
				llm_ids.push(entry_id.clone());
			}
		}
	}
	(tpl_ids, llm_ids)
}

/// Process template-eligible entries in batch.
///
/// Applies user comments, templates, or skip logic.
/// Returns the number of entries processed.
#[allow(clippy::print_stderr)]
pub(crate) fn process_template_batch(
	stores: &SharedDocStores,
	canonical: &PathBuf,
	tpl_ids: &[String],
	progress: &Arc<Mutex<DocGenProgress>>,
) -> usize {
	let mut count = 0;
	for entry_id in tpl_ids {
		if apply_template_single(
			stores, canonical, entry_id,
		) {
			count += 1;
			if let Ok(mut prog) = progress.lock() {
				prog.completed += 1;
			}
		}
	}
	eprintln!(
		"[daemon] Template docs: {} entries",
		count,
	);
	count
}

/// Apply template or user comment to a single entry.
///
/// Returns true if entry was updated successfully.
fn apply_template_single(
	stores: &SharedDocStores,
	canonical: &PathBuf,
	entry_id: &str,
) -> bool {
	let Ok(mut lock) = stores.lock() else {
		return false;
	};
	let Some(store) = lock.get_mut(canonical) else {
		return false;
	};
	let Some(entry) = store.get_mut(entry_id) else {
		return false;
	};

	let strategy = classify_entry(entry);
	let doc = match strategy {
		DocStrategy::UserComment => {
			entry.user_comment.clone().unwrap_or_default()
		}
		DocStrategy::Template => {
			build_template_doc(entry)
		}
		DocStrategy::Skip => {
			entry.code_snippet.clone()
		}
		DocStrategy::Llm => return false,
	};

	entry.mark_ready(doc);
	true
}

/// Build template doc choosing simple or composite.
fn build_template_doc(
	entry: &crate::retrieval::docgen::DocEntry,
) -> String {
	let simple = generate_template_doc(entry);
	if !simple.is_empty() {
		return simple;
	}
	generate_composite_doc(entry)
}
