use crate::domain::errors::atlassian::AtlassianResult;
use crate::domain::jira_detail::JiraIssueDetail;
use crate::service::atlassian::client_jira::JiraClient;

use super::detail_map;
use super::super::jira_types::JiraIssue;

/// Fields for the detail endpoint
const DETAIL_FIELDS: &str = "summary,status,\
	priority,issuetype,assignee,description,\
	customfield_10016,comment";

/// Fetch full issue detail by key.
/// Calls GET /issue/{key} with expanded fields
/// including description and comments.
pub fn fetch_issue_detail(
	client: &JiraClient,
	key: &str,
) -> AtlassianResult<JiraIssueDetail> {
	let path = format!(
		"/issue/{key}?fields={DETAIL_FIELDS}"
	);
	let json = client.get(&path)?;
	let issue: JiraIssue =
		serde_json::from_str(&json)?;
	Ok(detail_map::map_issue_detail(&issue))
}
