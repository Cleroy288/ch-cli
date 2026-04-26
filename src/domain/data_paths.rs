use std::path::{Path, PathBuf};

/// Old local dir name (for migration detection)
pub const LEGACY_DATA_DIR: &str = ".rustean-data";

/// Env var to override base dir (for tests)
const ENV_OVERRIDE: &str = "RUSTEAN_DATA_DIR";

/// App directory name under ~/.config/
const APP_DIR: &str = "rustean";

/// Subdirectory holding per-project data
const PROJECTS_DIR: &str = "projects";

/// 8-char blake3 hash of the canonical root path
pub fn project_hash(root: &Path) -> String {
	let canonical = root
		.canonicalize()
		.unwrap_or_else(|_| root.to_path_buf());
	let hash = blake3::hash(
		canonical.to_string_lossy().as_bytes(),
	);
	hash.to_hex()[..8].to_string()
}

/// ~/.config/rustean/ (or RUSTEAN_DATA_DIR override)
pub fn base_dir() -> PathBuf {
	if let Ok(dir) = std::env::var(ENV_OVERRIDE) {
		return PathBuf::from(dir);
	}
	default_base_dir()
}

/// Default base: ~/.config/rustean/
fn default_base_dir() -> PathBuf {
	dirs::config_dir()
		.unwrap_or_else(home_fallback)
		.join(APP_DIR)
}

/// Fallback when config_dir() returns None
fn home_fallback() -> PathBuf {
	dirs::home_dir()
		.unwrap_or_else(|| PathBuf::from("."))
		.join(".config")
}

/// Per-project data dir: base/projects/{hash}/
pub fn data_dir(root: &Path) -> PathBuf {
	base_dir()
		.join(PROJECTS_DIR)
		.join(project_hash(root))
}
