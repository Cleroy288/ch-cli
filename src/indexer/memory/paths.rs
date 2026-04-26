use std::path::{Path, PathBuf};

use crate::domain::data_paths_dirs;

const SESSIONS_DIR: &str = "sessions";
const TANTIVY_DIR: &str = "tantivy";
const META_FILE: &str = "meta.json";

pub fn memory_dir(root: &Path) -> PathBuf {
	data_paths_dirs::memory_dir(root)
}

pub fn sessions_dir(root: &Path) -> PathBuf {
	memory_dir(root).join(SESSIONS_DIR)
}

pub fn session_file(
	root: &Path,
	session_id: &str,
) -> PathBuf {
	sessions_dir(root)
		.join(format!("{session_id}.jsonl"))
}

pub fn tantivy_dir(root: &Path) -> PathBuf {
	memory_dir(root).join(TANTIVY_DIR)
}

pub fn meta_file(root: &Path) -> PathBuf {
	memory_dir(root).join(META_FILE)
}
