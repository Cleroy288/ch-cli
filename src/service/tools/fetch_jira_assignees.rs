use std::path::Path;

use crate::domain::errors::atlassian::AtlassianResult;
use crate::service::atlassian::client_jira::JiraClient;

use super::fetch_jira_assignees_query::{
    extract_issue_keys, run_raw_query,
};
use super::jira_project_cache;
use super::jira_types::USER_JQL;

/// Fetch unique assignee names from project(s).
/// Uses project_key if configured; otherwise
/// discovers keys from cache or lightweight API.
/// Returns sorted unique display names.
pub fn fetch_jira_assignees(
    client: &JiraClient,
    project_key: Option<&str>,
    root: &Path,
) -> AtlassianResult<Vec<String>> {
    let jql = match project_key {
        Some(key) => format!(
            "project = {key} \
            ORDER BY updated DESC"
        ),
        None => build_discover_jql(client, root)?,
    };
    let result = run_raw_query(client, &jql)?;
    let mut names: Vec<String> = result
        .issues
        .iter()
        .filter_map(|issue| {
            issue
                .fields
                .assignee
                .as_ref()
                .map(|who| who.display_name.clone())
        })
        .collect();
    names.sort();
    names.dedup();
    Ok(names)
}

fn build_discover_jql(
    client: &JiraClient,
    root: &Path,
) -> AtlassianResult<String> {
    let keys =
        resolve_keys_lightweight(client, root)?;
    if keys.is_empty() {
        return Ok(USER_JQL.to_string());
    }
    let list = keys.join(", ");
    Ok(format!(
        "project IN ({list}) \
        ORDER BY updated DESC"
    ))
}

/// Lightweight: only requests assignee field,
/// extracts project keys from issue keys.
fn resolve_keys_lightweight(
    client: &JiraClient,
    root: &Path,
) -> AtlassianResult<Vec<String>> {
    if let Some(cached) =
        jira_project_cache::load_project_keys(root)
    {
        return Ok(cached);
    }
    let result =
        run_raw_query(client, USER_JQL)?;
    let keys = extract_issue_keys(&result);
    if !keys.is_empty() {
        jira_project_cache::save_project_keys(
            root, &keys,
        );
    }
    Ok(keys)
}
