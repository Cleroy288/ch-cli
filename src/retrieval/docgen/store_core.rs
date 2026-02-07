//! DocStore core - struct definition and basic accessors.
//!
//! Provides the DocStore struct, its constructor,
//! and basic size/emptiness checks.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::indexer::state::INDEX_DIR_NAME;
use crate::retrieval::docgen::entry::DocEntry;

/// File name for doc store persistence.
pub(crate) const DOCS_FILE_NAME: &str = "docs.json";

/// Storage for generated documentation.
pub struct DocStore {
	/// all documentation entries (id -> entry)
	pub(crate) entries: HashMap<String, DocEntry>,
	/// path to the project root
	pub(crate) project_path: PathBuf,
	/// whether all docs have been generated
	pub(crate) generation_complete: bool,
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
	pub(crate) fn docs_file_path(&self) -> PathBuf {
		self.project_path
			.join(INDEX_DIR_NAME)
			.join(DOCS_FILE_NAME)
	}

	/// Get number of entries.
	pub fn len(&self) -> usize {
		self.entries.len()
	}

	/// Check if empty.
	pub fn is_empty(&self) -> bool {
		self.entries.is_empty()
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

