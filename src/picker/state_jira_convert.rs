use std::collections::BTreeSet;

use crate::domain::jira::{
    JiraBoardData, JiraTicketDetail,
};
use crate::domain::tool_ref::ToolItem;

const UNASSIGNED: &str = "Unassigned";

pub fn derive_tool_items(
    board: &JiraBoardData,
) -> Vec<ToolItem> {
    board
        .tickets
        .iter()
        .map(|tkt| ToolItem {
            key: tkt.key.clone(),
            display: format!(
                "{} - {}",
                tkt.key, tkt.summary,
            ),
            description: tkt.status.clone(),
        })
        .collect()
}

/// Extract unique sorted assignee names
pub fn extract_assignees(
    tickets: &[JiraTicketDetail],
) -> Vec<String> {
    tickets
        .iter()
        .map(|tkt| match tkt.assignee.is_empty() {
            true => UNASSIGNED.to_string(),
            false => tkt.assignee.clone(),
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Extract unique sorted project keys
pub fn extract_project_keys(
    tickets: &[JiraTicketDetail],
) -> Vec<String> {
    tickets
        .iter()
        .map(|tkt| tkt.project_key.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
