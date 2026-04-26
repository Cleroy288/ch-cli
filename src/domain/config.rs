use serde::{Deserialize, Serialize};

use super::agent::AgentPreference;
use super::backend_kind::BackendKind;
use super::credentials::{
	AikidoCredentials, AtlassianCredentials,
};
use super::repo_info::RepoCache;

#[derive(
	Debug, Clone, Default,
	Serialize, Deserialize,
)]
pub struct RusteanConfig {
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub credentials: Option<AtlassianCredentials>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub aikido: Option<AikidoCredentials>,
	#[serde(default)]
	pub claude: ClaudeConfig,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub agent: Option<AgentPreference>,
	#[serde(default)]
	pub cache: CacheConfig,
	#[serde(default)]
	pub setup: SetupState,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeConfig {
	/// Which CLI backend to use
	#[serde(default)]
	pub backend: BackendKind,
	/// "haiku", "sonnet", or "opus"
	#[serde(default = "default_model")]
	pub model: String,
	/// "low", "medium", "high", or "max"
	#[serde(default = "default_effort")]
	pub effort: String,
	/// Active session for conversation resume
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub session_id: Option<String>,
	#[serde(default)]
	pub integration: IntegrationMode,
}

#[derive(
	Debug, Clone, Default,
	Serialize, Deserialize, PartialEq, Eq,
)]
#[serde(rename_all = "snake_case")]
pub enum IntegrationMode {
	#[default]
	Cli,
	Api {
		api_key: String,
	},
}

#[derive(
	Debug, Clone, Default,
	Serialize, Deserialize,
)]
pub struct SetupState {
	#[serde(default)]
	pub credentials: bool,
	#[serde(default)]
	pub agent: bool,
	#[serde(default)]
	pub integration: bool,
	#[serde(default)]
	pub repo_scan: bool,
	#[serde(default)]
	pub indexing: bool,
}

#[derive(
	Debug, Clone, Default,
	Serialize, Deserialize,
)]
pub struct CacheConfig {
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub repos: Option<RepoCache>,
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub jira_project_keys: Option<Vec<String>>,
}

impl Default for ClaudeConfig {
	fn default() -> Self {
		Self {
			backend: BackendKind::default(),
			model: default_model(),
			effort: default_effort(),
			session_id: None,
			integration: IntegrationMode::default(),
		}
	}
}

fn default_model() -> String {
	"sonnet".to_string()
}

fn default_effort() -> String {
	crate::domain::effort::DEFAULT_EFFORT.to_string()
}
