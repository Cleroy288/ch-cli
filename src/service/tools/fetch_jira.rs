use std::path::Path;

use percent_encoding::{
    utf8_percent_encode, NON_ALPHANUMERIC,
};

use crate::domain::errors::atlassian::AtlassianResult;
use crate::domain::jira::JiraBoardData;
use crate::service::atlassian::client_jira::JiraClient;

use super::jira_map;
use super::jira_types::JiraSearchResult;

/// Includes sprint (customfield_10021) for
/// per-ticket sprint grouping in the UI.
const JIRA_FIELDS: &str = "summary,status,\
    priority,issuetype,assignee,\
    customfield_10016,customfield_10021";

/// With project_key: single-project fetch.
/// Without: auto-discover (cached) from user tickets.
pub fn fetch_jira_board(
    client: &JiraClient,
    project_key: Option<&str>,
    root: &Path,
    assignee: Option<&str>,
) -> AtlassianResult<JiraBoardData> {
    match project_key {
        Some(key) => {
            let jql =
                build_project_jql(key, assignee);
            run_jql(client, &jql)
        }
        None => {
            super::fetch_jira_discover
                ::discover_project_board(
                    client, root, assignee,
                )
        }
    }
}

pub fn run_jql(
    client: &JiraClient,
    jql: &str,
) -> AtlassianResult<JiraBoardData> {
    let encoded = utf8_percent_encode(
        jql, NON_ALPHANUMERIC,
    );
    let path = format!(
        "/search/jql?jql={encoded}\
        &maxResults=50&fields={JIRA_FIELDS}"
    );
    let json = client.get(&path)?;
    let result: JiraSearchResult =
        serde_json::from_str(&json)?;
    Ok(jira_map::map_board(result.issues))
}

pub fn build_project_jql(
    key: &str,
    assignee: Option<&str>,
) -> String {
    let filter = assignee
        .map(|name| {
            format!(
                " AND assignee = \"{name}\""
            )
        })
        .unwrap_or_default();
    format!(
        "project = {key}{filter} \
        ORDER BY updated DESC"
    )
}
