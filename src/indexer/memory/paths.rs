//! Path helpers for the memory system.
//!
//! All memory data lives under `{root}/.rustean-memory/`.

use std::path::{Path, PathBuf};

/// Name of the memory root directory
const MEMORY_DIR: &str = ".rustean-memory";

/// Name of the sessions subdirectory
const SESSIONS_DIR: &str = "sessions";

/// Name of the Tantivy index subdirectory
const TANTIVY_DIR: &str = "tantivy";

/// Name of the metadata/stats file
const META_FILE: &str = "meta.json";

/// Root memory directory for a project
pub fn memory_dir(root: &Path) -> PathBuf {
	root.join(MEMORY_DIR)
}

/// Directory containing session JSONL files
pub fn sessions_dir(root: &Path) -> PathBuf {
	memory_dir(root).join(SESSIONS_DIR)
}

/// JSONL file for a specific session
pub fn session_file(
	root: &Path,
	session_id: &str,
) -> PathBuf {
	sessions_dir(root)
		.join(format!("{session_id}.jsonl"))
}

/// Directory for the Tantivy search index
pub fn tantivy_dir(root: &Path) -> PathBuf {
	memory_dir(root).join(TANTIVY_DIR)
}

/// Path to the stats metadata file
pub fn meta_file(root: &Path) -> PathBuf {
	memory_dir(root).join(META_FILE)
}
