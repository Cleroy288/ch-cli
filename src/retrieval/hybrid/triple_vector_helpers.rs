//! Triple Vector Store — Helper Functions
//!
//! Utility functions for triple vector store initialization.

use std::path::Path;

/// Create parent directories for store files
pub(crate) fn create_store_dirs(
	code_path: &Path,
	doc_path: &Path,
	notes_path: &Path,
) -> std::io::Result<()> {
	if let Some(parent) = code_path.parent() {
		std::fs::create_dir_all(parent)?;
	}
	if let Some(parent) = doc_path.parent() {
		std::fs::create_dir_all(parent)?;
	}
	if let Some(parent) = notes_path.parent() {
		std::fs::create_dir_all(parent)?;
	}
	Ok(())
}
