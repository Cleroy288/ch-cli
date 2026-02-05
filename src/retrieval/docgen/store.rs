//! DocStore - Storage for generated documentation.
//!
//! Provides persistence, querying, and status tracking for documentation entries.

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use crate::indexer::state::INDEX_DIR_NAME;
use crate::indexer::{IndexState, Symbol};
use crate::retrieval::docgen::{DocEntry, DocStatus};

/// File name for doc store persistence.
const DOCS_FILE_NAME: &str = "docs.json";

/// Storage for generated documentation.
pub struct DocStore {
	/// all documentation entries (id -> entry)
	entries: HashMap<String, DocEntry>,
	/// path to the project root
	project_path: PathBuf,
	/// whether all docs have been generated
	generation_complete: bool,
}

impl DocStore {
	/// Create new store for a project.
	pub fn new(project_path: &Path) -> Self {
		Self {
			entries: HashMap::new(),
			project_path: project_path.to_path_buf(),
			generation_complete: false,
		}
	}

	/// Get path to docs.json file.
	fn docs_file_path(&self) -> PathBuf {
		self.project_path.join(INDEX_DIR_NAME).join(DOCS_FILE_NAME)
	}

	/// Load store from disk.
	pub fn load(project_path: &Path) -> io::Result<Self> {
		let docs_file = project_path.join(INDEX_DIR_NAME).join(DOCS_FILE_NAME);

		if !docs_file.exists() {
			return Ok(Self::new(project_path));
		}

		let content = fs::read_to_string(&docs_file)?;
		let entries: HashMap<String, DocEntry> = serde_json::from_str(&content)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

		let generation_complete = entries.values().all(|e| e.status == DocStatus::Ready);

		Ok(Self {
			entries,
			project_path: project_path.to_path_buf(),
			generation_complete,
		})
	}

	/// Save store to disk.
	pub fn save(&self) -> io::Result<()> {
		let index_dir = self.project_path.join(INDEX_DIR_NAME);
		fs::create_dir_all(&index_dir)?;

		let docs_file = self.docs_file_path();
		let content = serde_json::to_string_pretty(&self.entries)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

		fs::write(&docs_file, content)?;
		Ok(())
	}

	/// Check if store exists on disk.
	pub fn exists(project_path: &Path) -> bool {
		project_path.join(INDEX_DIR_NAME).join(DOCS_FILE_NAME).exists()
	}

	/// Add or update an entry.
	pub fn upsert(&mut self, entry: DocEntry) {
		self.entries.insert(entry.id.clone(), entry);
		self.update_completion_status();
	}

	/// Get entry by ID.
	pub fn get(&self, id: &str) -> Option<&DocEntry> {
		self.entries.get(id)
	}

	/// Get mutable entry by ID.
	pub fn get_mut(&mut self, id: &str) -> Option<&mut DocEntry> {
		self.entries.get_mut(id)
	}

	/// Get entry by symbol name (first match).
	pub fn get_by_name(&self, name: &str) -> Option<&DocEntry> {
		self.entries.values().find(|e| e.name == name)
	}

	/// Get all entries.
	pub fn all_entries(&self) -> impl Iterator<Item = &DocEntry> {
		self.entries.values()
	}

	/// Get all entries (mutable).
	pub fn all_entries_mut(&mut self) -> impl Iterator<Item = &mut DocEntry> {
		self.entries.values_mut()
	}

	/// Get all pending entries.
	pub fn get_pending(&self) -> Vec<&DocEntry> {
		self.entries
			.values()
			.filter(|e| e.status == DocStatus::Pending)
			.collect()
	}

	/// Get all pending entry IDs.
	pub fn get_pending_ids(&self) -> Vec<String> {
		self.entries
			.values()
			.filter(|e| e.status == DocStatus::Pending)
			.map(|e| e.id.clone())
			.collect()
	}

	/// Check if all docs are ready.
	pub fn is_ready(&self) -> bool {
		self.generation_complete
	}

	/// Update completion status.
	fn update_completion_status(&mut self) {
		self.generation_complete = !self.entries.is_empty()
			&& self.entries.values().all(|e| {
				e.status == DocStatus::Ready || e.status == DocStatus::Failed
			});
	}

	/// Get generation statistics.
	pub fn stats(&self) -> DocStoreStats {
		let total = self.entries.len();
		let ready = self.entries.values().filter(|e| e.status == DocStatus::Ready).count();
		let pending = self.entries.values().filter(|e| e.status == DocStatus::Pending).count();
		let generating = self.entries.values().filter(|e| e.status == DocStatus::Generating).count();
		let failed = self.entries.values().filter(|e| e.status == DocStatus::Failed).count();

		DocStoreStats {
			total,
			ready,
			pending,
			generating,
			failed,
			is_complete: self.generation_complete,
		}
	}

	/// Get number of entries.
	pub fn len(&self) -> usize {
		self.entries.len()
	}

	/// Check if empty.
	pub fn is_empty(&self) -> bool {
		self.entries.is_empty()
	}

	/// Clear all entries.
	pub fn clear(&mut self) {
		self.entries.clear();
		self.generation_complete = false;
	}

	/// Remove entry by ID.
	pub fn remove(&mut self, id: &str) -> Option<DocEntry> {
		let entry = self.entries.remove(id);
		self.update_completion_status();
		entry
	}

	/// Sync with index state, mark stale entries.
	pub fn sync_with_index(&mut self, index_state: &IndexState) {
		for (path, file_state) in &index_state.files {
			let mtime = file_state.mtime;

			for entry in self.entries.values_mut() {
				if entry.file_path == *path && entry.is_stale(mtime) {
					entry.status = DocStatus::Pending;
					entry.llm_doc = None;
					entry.doc_embedding = None;
				}
			}
		}
		self.update_completion_status();
	}

	/// Create entries from symbols (without generating docs).
	pub fn populate_from_symbols(&mut self, symbols: &[Symbol]) {
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

	/// Get entries for a specific file.
	pub fn get_by_file(&self, file_path: &Path) -> Vec<&DocEntry> {
		self.entries
			.values()
			.filter(|e| e.file_path == file_path)
			.collect()
	}

	/// Search entries by name pattern.
	pub fn search_by_name(&self, pattern: &str) -> Vec<&DocEntry> {
		let pattern_lower = pattern.to_lowercase();
		self.entries
			.values()
			.filter(|e| e.name.to_lowercase().contains(&pattern_lower))
			.collect()
	}
}

/// Statistics about the doc store.
#[derive(Debug, Clone)]
pub struct DocStoreStats {
	/// total number of entries
	pub total: usize,
	/// entries with Ready status
	pub ready: usize,
	/// entries with Pending status
	pub pending: usize,
	/// entries with Generating status
	pub generating: usize,
	/// entries with Failed status
	pub failed: usize,
	/// whether generation is complete
	pub is_complete: bool,
}

impl DocStoreStats {
	/// Get completion percentage.
	pub fn completion_percent(&self) -> f64 {
		if self.total == 0 {
			return 100.0;
		}
		(self.ready as f64 / self.total as f64) * 100.0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::indexer::SymbolKind;
	use tempfile::tempdir;

	#[test]
	fn test_store_new() {
		let dir = tempdir().unwrap();
		let store = DocStore::new(dir.path());

		assert!(store.is_empty());
		assert!(!store.is_ready());
	}

	#[test]
	fn test_store_upsert_and_get() {
		let dir = tempdir().unwrap();
		let mut store = DocStore::new(dir.path());

		let entry = DocEntry::new(
			"test_fn".to_string(),
			SymbolKind::Function,
			PathBuf::from("src/main.rs"),
			10,
		);
		let id = entry.id.clone();

		store.upsert(entry);

		assert_eq!(store.len(), 1);
		assert!(store.get(&id).is_some());
		assert_eq!(store.get(&id).unwrap().name, "test_fn");
	}

	#[test]
	fn test_store_pending() {
		let dir = tempdir().unwrap();
		let mut store = DocStore::new(dir.path());

		let entry1 = DocEntry::new(
			"fn1".to_string(),
			SymbolKind::Function,
			PathBuf::from("src/main.rs"),
			10,
		);
		let mut entry2 = DocEntry::new(
			"fn2".to_string(),
			SymbolKind::Function,
			PathBuf::from("src/main.rs"),
			20,
		);
		entry2.mark_ready("doc".to_string());

		store.upsert(entry1);
		store.upsert(entry2);

		let pending = store.get_pending();
		assert_eq!(pending.len(), 1);
		assert_eq!(pending[0].name, "fn1");
	}

	#[test]
	fn test_store_stats() {
		let dir = tempdir().unwrap();
		let mut store = DocStore::new(dir.path());

		let entry1 = DocEntry::new(
			"fn1".to_string(),
			SymbolKind::Function,
			PathBuf::from("src/main.rs"),
			10,
		);
		let mut entry2 = DocEntry::new(
			"fn2".to_string(),
			SymbolKind::Function,
			PathBuf::from("src/main.rs"),
			20,
		);
		entry2.mark_ready("doc".to_string());

		store.upsert(entry1);
		store.upsert(entry2);

		let stats = store.stats();
		assert_eq!(stats.total, 2);
		assert_eq!(stats.ready, 1);
		assert_eq!(stats.pending, 1);
		assert!(!stats.is_complete);
	}

	#[test]
	fn test_store_persistence() {
		let dir = tempdir().unwrap();

		{
			let mut store = DocStore::new(dir.path());
			let mut entry = DocEntry::new(
				"test".to_string(),
				SymbolKind::Function,
				PathBuf::from("src/main.rs"),
				10,
			);
			entry.mark_ready("Generated doc".to_string());
			store.upsert(entry);
			store.save().unwrap();
		}

		{
			let store = DocStore::load(dir.path()).unwrap();
			assert_eq!(store.len(), 1);
			let entry = store.get_by_name("test").unwrap();
			assert_eq!(entry.llm_doc, Some("Generated doc".to_string()));
			assert!(store.is_ready());
		}
	}
}
