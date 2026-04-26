use std::path::Path;

use crate::domain::agent::AgentPreference;
use crate::domain::config::{
	ClaudeConfig, IntegrationMode,
};
use crate::domain::repo_info::RepoCache;
use crate::service::atlassian::types
	::AtlassianCredentials;

use super::migration_config_readers::{
	read_json, read_model, read_session_id,
};

pub fn load_old_creds(
	data: &Path,
) -> Option<AtlassianCredentials> {
	read_json(data, super::migration_config::OLD_CREDS)
}

pub fn build_claude_config(
	data: &Path,
) -> ClaudeConfig {
	let model = read_model(data);
	let session_id = read_session_id(data);
	ClaudeConfig {
		backend: Default::default(),
		model,
		effort: crate::domain::effort::DEFAULT_EFFORT
			.to_string(),
		session_id,
		integration: IntegrationMode::Cli,
	}
}

pub fn load_old_agent(
	data: &Path,
) -> Option<AgentPreference> {
	read_json(data, super::migration_config::OLD_AGENT)
}

pub fn load_old_repos(
	data: &Path,
) -> Option<RepoCache> {
	read_json(data, super::migration_config::OLD_REPOS)
}

pub fn load_old_jira(
	data: &Path,
) -> Option<Vec<String>> {
	let keys: Vec<String> =
		read_json(data, super::migration_config::OLD_JIRA)?;
	if keys.is_empty() { None } else { Some(keys) }
}
