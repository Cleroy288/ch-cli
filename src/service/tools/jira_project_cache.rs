use std::path::Path;

use crate::service::config;

/// Returns None if no keys are stored.
pub fn load_project_keys(
	root: &Path,
) -> Option<Vec<String>> {
	config::load_config(root)
		.cache
		.jira_project_keys
}

/// Best-effort: silently ignores failures.
pub fn save_project_keys(
	root: &Path,
	keys: &[String],
) {
	let owned = keys.to_vec();
	let _ = config::update_config(
		root,
		move |cfg| {
			cfg.cache.jira_project_keys =
				Some(owned);
		},
	);
}
