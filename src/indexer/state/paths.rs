use std::path::{Path, PathBuf};

use crate::domain::data_paths_dirs;

use super::types::IndexState;

impl IndexState {
	pub fn index_dir(root: &Path) -> PathBuf {
		data_paths_dirs::index_dir(root)
	}

	pub fn state_file(root: &Path) -> PathBuf {
		Self::index_dir(root).join("state.json")
	}

	pub fn tantivy_dir(root: &Path) -> PathBuf {
		Self::index_dir(root).join("tantivy")
	}

	pub fn refs_dir(root: &Path) -> PathBuf {
		Self::index_dir(root).join("refs")
	}

	pub fn refs_file(root: &Path) -> PathBuf {
		Self::index_dir(root).join("refs.json")
	}

	pub fn trigram_file(root: &Path) -> PathBuf {
		Self::index_dir(root).join("trigrams.json")
	}
}
