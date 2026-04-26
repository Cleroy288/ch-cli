use std::path::{Path, PathBuf};

use super::data_paths;

/// Index storage: data_dir/index/
pub fn index_dir(root: &Path) -> PathBuf {
	data_paths::data_dir(root).join("index")
}

/// Memory storage: data_dir/memory/
pub fn memory_dir(root: &Path) -> PathBuf {
	data_paths::data_dir(root).join("memory")
}

/// Per-project config: data_dir/config.json
pub fn config_file(root: &Path) -> PathBuf {
	data_paths::data_dir(root).join("config.json")
}

/// Global config: base/config.json
pub fn global_config_file() -> PathBuf {
	data_paths::base_dir().join("config.json")
}

/// Manifest: base/projects.json
pub fn manifest_file() -> PathBuf {
	data_paths::base_dir().join("projects.json")
}

/// Prompt history: data_dir/history.json
pub fn history_file(root: &Path) -> PathBuf {
	data_paths::data_dir(root).join("history.json")
}
