use std::io;
use std::path::Path;

use crate::domain::agent::AgentPreference;
use crate::service::config;

pub fn load_agent(
	root: &Path,
) -> Option<AgentPreference> {
	config::load_config(root).agent
}

pub fn save_agent(
	root: &Path,
	pref: &AgentPreference,
) -> io::Result<()> {
	let owned = pref.clone();
	config::update_config(root, move |cfg| {
		cfg.agent = Some(owned);
	})
}
