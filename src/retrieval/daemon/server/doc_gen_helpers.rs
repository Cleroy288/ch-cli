//! Helper functions for doc generation batch processing.

use std::path::PathBuf;
use std::sync::{Arc, Mutex};

use super::types::{
	SharedDocGenerator, SharedDocStores,
};
use super::DocGenProgress;
use crate::retrieval::docgen::DocEntry;

/// Run LLM generation on a single entry
///
/// Returns None if generator is gone (cancelled).
#[allow(clippy::print_stderr)]
pub(crate) fn generate_single(
	gen: &SharedDocGenerator,
	entry: &mut DocEntry,
	prompt: &str,
) -> Option<bool> {
	let Ok(mut lock) = gen.lock() else {
		return None;
	};
	let generator = (*lock).as_mut()?;
	match generator.generate_from_prompt(entry, prompt) {
		Ok(()) => Some(true),
		Err(err) => {
			eprintln!(
				"[daemon] Doc gen error for {}: {}",
				entry.name, err
			);
			Some(false)
		}
	}
}

/// Write generated entry back to store
pub(crate) fn write_entry_back(
	stores: &SharedDocStores,
	canonical: &PathBuf,
	entry: DocEntry,
) {
	let Ok(mut lock) = stores.lock() else {
		return;
	};
	if let Some(store) = lock.get_mut(canonical) {
		store.upsert(entry);
	}
}

/// Update the progress counters
pub(crate) fn update_progress(
	progress: &Arc<Mutex<DocGenProgress>>,
	success: bool,
) {
	let Ok(mut prog) = progress.lock() else {
		return;
	};
	prog.completed += 1;
	if !success {
		prog.failed += 1;
	}
}

/// Save store to disk periodically
#[allow(clippy::print_stderr)]
pub(crate) fn periodic_save(
	stores: &SharedDocStores,
	canonical: &PathBuf,
) {
	let Ok(lock) = stores.lock() else {
		return;
	};
	let Some(store) = lock.get(canonical) else {
		return;
	};
	if let Err(err) = store.save() {
		eprintln!("[daemon] Failed to save: {}", err);
	}
}

/// Unload doc generator model to free GPU memory
///
/// Sets the shared generator to None, dropping the
/// Qwen2.5 model (~500MB GPU). Reloaded on next use.
#[allow(clippy::print_stderr)]
pub(crate) fn unload_generator(
	generator: &SharedDocGenerator,
) {
	let Ok(mut lock) = generator.lock() else {
		return;
	};
	if lock.is_some() {
		*lock = None;
		eprintln!(
			"[daemon] Doc generation model unloaded \
			 (freed ~500MB GPU)"
		);
	}
}
