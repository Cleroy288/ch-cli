use std::collections::HashSet;
use std::path::Path;

use crate::domain::errors::atlassian::AtlassianResult;
use crate::domain::jira::JiraBoardData;
use crate::service::atlassian::client_jira::JiraClient;

use super::fetch_jira::run_jql;
use super::jira_project_cache;
use super::jira_types::USER_JQL;

/// Uses cached keys if available; otherwise
/// discovers from user tickets and caches.
pub fn discover_project_board(
    client: &JiraClient,
    root: &Path,
    assignee: Option<&str>,
) -> AtlassianResult<JiraBoardData> {
    let keys = resolve_project_keys(client, root)?;
    if keys.is_empty() {
        return run_jql(client, USER_JQL);
    }
    let jql =
        build_multi_project_jql(&keys, assignee);
    run_jql(client, &jql)
}

fn resolve_project_keys(
    client: &JiraClient,
    root: &Path,
) -> AtlassianResult<Vec<String>> {
    if let Some(cached) =
        jira_project_cache::load_project_keys(root)
    {
        return Ok(cached);
    }
    let board = run_jql(client, USER_JQL)?;
    let keys = extract_project_keys(&board);
    if !keys.is_empty() {
        jira_project_cache::save_project_keys(
            root, &keys,
        );
    }
    Ok(keys)
}

/// "EVB-123" -> "EVB", "FOO-45" -> "FOO"
pub fn extract_project_keys(
    board: &JiraBoardData,
) -> Vec<String> {
    let mut seen = HashSet::new();
    board
        .tickets
        .iter()
        .filter_map(|tkt| {
            tkt.key.split('-').next()
                .filter(|p| !p.is_empty())
        })
        .filter(|p| seen.insert(p.to_string()))
        .map(String::from)
        .collect()
}

pub fn build_multi_project_jql(
    keys: &[String],
    assignee: Option<&str>,
) -> String {
    let list = keys.join(", ");
    let filter = assignee
        .map(|name| {
            format!(
                " AND assignee = \"{name}\""
            )
        })
        .unwrap_or_default();
    format!(
        "project IN ({list}){filter} \
        ORDER BY updated DESC"
    )
}
