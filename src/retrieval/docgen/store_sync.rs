//! DocStore synchronization with the code index.
//!
//! Handles syncing entries with IndexState to detect
//! stale docs, and populating entries from symbols.

use crate::indexer::{IndexState, Symbol};
use crate::retrieval::docgen::entry::DocEntry;
use crate::retrieval::docgen::entry_types::DocStatus;
use crate::retrieval::docgen::store_core::DocStore;

/// Sync and populate operations.
impl DocStore {
	/// Sync with index state, mark stale entries.
	pub fn sync_with_index(
		&mut self,
		index_state: &IndexState,
	) {
		for (path, file_state) in &index_state.files {
			self.mark_stale_entries(
				path, file_state.mtime,
			);
		}
		self.update_completion_status();
	}

	/// Mark entries as stale if file was modified
	fn mark_stale_entries(
		&mut self,
		path: &std::path::Path,
		mtime: u64,
	) {
		for entry in self.entries.values_mut() {
			if entry.file_path != *path {
				continue;
			}
			if !entry.is_stale(mtime) {
				continue;
			}
			entry.status = DocStatus::Pending;
			entry.llm_doc = None;
			entry.doc_embedding = None;
		}
	}

	/// Create entries from symbols (without docs).
	pub fn populate_from_symbols(
		&mut self,
		symbols: &[Symbol],
	) {
		for symbol in symbols {
			let entry_id = DocEntry::generate_id(
				&symbol.location.file,
				&symbol.name,
				symbol.location.line,
			);

			if self.entries.contains_key(&entry_id) {
				continue;
			}

			let mut entry = DocEntry::new(
				symbol.name.clone(),
				symbol.kind,
				symbol.location.file.clone(),
				symbol.location.line,
			);

			if let Some(ref doc) = symbol.doc_comment {
				entry.user_comment = Some(doc.clone());
			}
			if let Some(ref sig) = symbol.signature {
				entry.signature = Some(sig.clone());
			}

			self.entries.insert(entry_id, entry);
		}
		self.update_completion_status();
	}
}
