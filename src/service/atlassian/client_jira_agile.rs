use crate::domain::errors::atlassian::AtlassianResult;

use super::client_jira::JiraClient;

/// Agile API path via the Atlassian gateway.
/// Through api.atlassian.com, the Agile API is
/// nested under /rest/api/2 -- NOT /rest/agile/1.0.
const JIRA_AGILE_GATEWAY: &str =
	"/rest/api/2/agile/1.0";
/// Agile API path for direct site URL access
const JIRA_AGILE_DIRECT: &str =
	"/rest/agile/1.0";
/// Base URL template for Jira Cloud API
const JIRA_API_BASE: &str =
	"https://api.atlassian.com/ex/jira";

/// Try Agile API via api.atlassian.com gateway
pub fn try_agile_gateway(
	client: &JiraClient,
	path: &str,
) -> AtlassianResult<String> {
	let url = format!(
		"{JIRA_API_BASE}/{}{JIRA_AGILE_GATEWAY}\
		{path}",
		client.creds.cloud_id,
	);
	client.do_get(&url, "Agile API (gateway)")
}

/// Try Agile API via direct site URL
pub fn try_agile_direct(
	client: &JiraClient,
	path: &str,
) -> AtlassianResult<String> {
	let base =
		client.creds.base_url.trim_end_matches('/');
	let url = format!(
		"{base}{JIRA_AGILE_DIRECT}{path}",
	);
	client.do_get(&url, "Agile API (direct)")
}
