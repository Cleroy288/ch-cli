use std::io;
use std::path::Path;

use crate::domain::config::RusteanConfig;
use crate::domain::data_paths_dirs;

use super::load::load_config;

pub fn save_config(
	root: &Path,
	config: &RusteanConfig,
) -> io::Result<()> {
	let path = data_paths_dirs::config_file(root);
	ensure_parent(&path)?;
	let json =
		serde_json::to_string_pretty(config)
			.map_err(io::Error::other)?;
	std::fs::write(&path, json)
}

/// Atomic read-modify-write
/// Loads current config, applies the closure,
/// then writes back. Creates default if missing.
pub fn update_config<F>(
	root: &Path,
	mutate: F,
) -> io::Result<()>
where
	F: FnOnce(&mut RusteanConfig),
{
	let mut config = load_config(root);
	mutate(&mut config);
	save_config(root, &config)
}

/// Create parent directory if needed
fn ensure_parent(
	path: &Path,
) -> io::Result<()> {
	if let Some(parent) = path.parent() {
		std::fs::create_dir_all(parent)?;
	}
	Ok(())
}
