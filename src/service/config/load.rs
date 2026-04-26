use std::path::Path;

use crate::domain::config::RusteanConfig;
use crate::domain::data_paths_dirs;

pub fn load_config(
	root: &Path,
) -> RusteanConfig {
	try_load_config(root)
		.unwrap_or_default()
}

pub fn try_load_config(
	root: &Path,
) -> Option<RusteanConfig> {
	let path = data_paths_dirs::config_file(root);
	let data =
		std::fs::read_to_string(&path).ok()?;
	serde_json::from_str(&data).ok()
}
