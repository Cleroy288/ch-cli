use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BbCredentials {
	pub email: String,
	pub token: String,
	pub workspace: String,
	/// Auto-detected from git remote
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub repo_slug: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JiraCredentials {
	pub email: String,
	pub token: String,
	/// e.g. https://x.atlassian.net
	pub base_url: String,
	/// Atlassian cloud instance ID
	pub cloud_id: String,
	/// e.g. "EVB"
	#[serde(default)]
	#[serde(skip_serializing_if = "Option::is_none")]
	pub project_key: Option<String>,
}

/// Aikido Security OAuth2 credentials
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AikidoCredentials {
	pub client_id: String,
	pub client_secret: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtlassianCredentials {
	pub bitbucket: Option<BbCredentials>,
	pub jira: Option<JiraCredentials>,
}
