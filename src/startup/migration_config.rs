use std::fs;
use std::path::Path;

use crate::domain::config::RusteanConfig;
use crate::domain::data_paths;
use crate::domain::data_paths_dirs;

use super::migration_config_loaders::{
	build_claude_config, load_old_agent,
	load_old_creds, load_old_jira, load_old_repos,
};

pub(crate) const OLD_CREDS: &str =
	"credentials.json";
pub(crate) const OLD_MODEL: &str =
	"claude_model.json";
pub(crate) const OLD_SESSION: &str =
	"claude_session.json";
pub(crate) const OLD_AGENT: &str = "agent.json";
pub(crate) const OLD_REPOS: &str = "repos.json";
pub(crate) const OLD_JIRA: &str =
	"jira_projects.json";

pub fn migrate_to_unified_config(
	root: &Path,
) {
	let config_path =
		data_paths_dirs::config_file(root);
	if config_path.exists() {
		return;
	}
	let data = data_paths::data_dir(root);
	if !has_any_old_file(&data) {
		return;
	}
	let config = build_config(&data);
	write_config(&config_path, &config);
	delete_old_files(&data);
}

fn has_any_old_file(data: &Path) -> bool {
	let names = [
		OLD_CREDS, OLD_MODEL, OLD_SESSION,
		OLD_AGENT, OLD_REPOS, OLD_JIRA,
	];
	names.iter().any(|n| data.join(n).exists())
}

fn build_config(data: &Path) -> RusteanConfig {
	let mut config = RusteanConfig::default();
	config.credentials = load_old_creds(data);
	config.claude = build_claude_config(data);
	config.agent = load_old_agent(data);
	config.cache.repos = load_old_repos(data);
	config.cache.jira_project_keys =
		load_old_jira(data);
	config
}

fn write_config(
	path: &Path,
	config: &RusteanConfig,
) {
	let Ok(json) =
		serde_json::to_string_pretty(config)
	else {
		return;
	};
	let _ = fs::write(path, json);
}

fn delete_old_files(data: &Path) {
	let names = [
		OLD_CREDS, OLD_MODEL, OLD_SESSION,
		OLD_AGENT, OLD_REPOS, OLD_JIRA,
	];
	for name in names {
		let _ = fs::remove_file(data.join(name));
	}
}
