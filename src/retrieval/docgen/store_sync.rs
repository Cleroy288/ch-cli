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
			let mtime = file_state.mtime;

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
		self.update_completion_status();
	}

	/// Create entries from symbols (without docs).
	pub fn populate_from_symbols(
		&mut self,
		symbols: &[Symbol],
	) {
		for symbol in symbols {
			let id = DocEntry::generate_id(
				&symbol.location.file,
				&symbol.name,
				symbol.location.line,
			);

			if self.entries.contains_key(&id) {
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

			self.entries.insert(id, entry);
		}
		self.update_completion_status();
	}
}

