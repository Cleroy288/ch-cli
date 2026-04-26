use percent_encoding::{
    utf8_percent_encode, NON_ALPHANUMERIC,
};
use serde::Deserialize;

use crate::domain::errors::atlassian::AtlassianResult;
use crate::service::atlassian::client_jira::JiraClient;

use super::jira_types::JiraAssignee;

/// Only fetch assignee field (very lightweight)
const ASSIGNEE_FIELD: &str = "assignee";

/// Max results for assignee discovery
const MAX_RESULTS: u32 = 100;

/// Lightweight search result (assignee only)
#[derive(Deserialize)]
pub struct AssigneeResult {
    pub issues: Vec<AssigneeIssue>,
}

/// Issue with key + assignee fields only
#[derive(Deserialize)]
pub struct AssigneeIssue {
    pub key: String,
    pub fields: AssigneeFields,
}

#[derive(Deserialize)]
pub struct AssigneeFields {
    #[serde(default)]
    pub assignee: Option<JiraAssignee>,
}

/// Extract unique project keys from issue keys.
/// "EVB-123" -> "EVB", sorted and deduplicated.
pub fn extract_issue_keys(
    result: &AssigneeResult,
) -> Vec<String> {
    let mut keys: Vec<String> = result
        .issues
        .iter()
        .filter_map(|issue| {
            issue
                .key
                .split('-')
                .next()
                .filter(|p| !p.is_empty())
                .map(String::from)
        })
        .collect();
    keys.sort();
    keys.dedup();
    keys
}

/// Raw API call with assignee-only fields
pub fn run_raw_query(
    client: &JiraClient,
    jql: &str,
) -> AtlassianResult<AssigneeResult> {
    let encoded = utf8_percent_encode(
        jql, NON_ALPHANUMERIC,
    )
    .to_string();
    let path = format!(
        "/search/jql?jql={encoded}\
        &maxResults={MAX_RESULTS}\
        &fields={ASSIGNEE_FIELD}"
    );
    let json = client.get(&path)?;
    Ok(serde_json::from_str(&json)?)
}
