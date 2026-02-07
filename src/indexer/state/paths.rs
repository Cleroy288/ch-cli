//! Path utilities for index state management.
//!
//! Provides methods for computing index-related paths.

use std::path::{Path, PathBuf};

use super::INDEX_DIR_NAME;
use super::types::IndexState;

impl IndexState {
	/// Get the index directory path
	pub fn index_dir(root: &Path) -> PathBuf {
		root.join(INDEX_DIR_NAME)
	}

	/// Get the state file path
	pub fn state_file(root: &Path) -> PathBuf {
		Self::index_dir(root).join("state.json")
	}

	/// Get the Tantivy index directory
	pub fn tantivy_dir(root: &Path) -> PathBuf {
		Self::index_dir(root).join("tantivy")
	}

	/// Get the references cache file path
	pub fn refs_file(root: &Path) -> PathBuf {
		Self::index_dir(root).join("refs.json")
	}

	/// Get the trigram index file path
	pub fn trigram_file(root: &Path) -> PathBuf {
		Self::index_dir(root).join("trigrams.json")
	}
}
