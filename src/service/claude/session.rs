use std::path::Path;

use crate::service::config;

pub fn save_session(
	project: &str,
	session_id: &str,
) {
	let root = Path::new(project);
	let owned = session_id.to_string();
	let _ = config::update_config(root, move |cfg| {
		cfg.claude.session_id = Some(owned);
	});
}

/// Returns None if no session is stored.
pub fn load_session(
	project: &str,
) -> Option<String> {
	let root = Path::new(project);
	config::load_config(root).claude.session_id
}

pub fn clear_session(project: &str) {
	let root = Path::new(project);
	let _ = config::update_config(root, |cfg| {
		cfg.claude.session_id = None;
	});
}
