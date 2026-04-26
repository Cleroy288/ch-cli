use std::io;
use std::path::Path;

use super::types::AtlassianCredentials;
use crate::service::config;

/// Returns None if not configured.
pub fn load_credentials(
	root: &Path,
) -> Option<AtlassianCredentials> {
	config::load_config(root).credentials
}

/// Merges into existing config via update.
pub fn save_credentials(
	root: &Path,
	creds: &AtlassianCredentials,
) -> io::Result<()> {
	let owned = creds.clone();
	config::update_config(root, move |cfg| {
		cfg.credentials = Some(owned);
	})
}

/// Returns true if config.json has credentials.
pub fn has_credentials(root: &Path) -> bool {
	load_credentials(root).is_some()
}
