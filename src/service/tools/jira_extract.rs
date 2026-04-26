use super::jira_types::JiraFields;

pub fn extract_type(
    fields: &JiraFields,
) -> String {
    fields
        .issuetype
        .as_ref()
        .map(|typ| typ.name.clone())
        .unwrap_or_else(|| "Task".into())
}

pub fn extract_assignee(
    fields: &JiraFields,
) -> String {
    fields
        .assignee
        .as_ref()
        .map(|who| who.display_name.clone())
        .unwrap_or_else(|| "Unassigned".into())
}

/// Jira returns sprints oldest-to-newest;
/// the last entry is the current/most recent.
pub fn extract_sprint_name(
    fields: &JiraFields,
) -> String {
    fields
        .sprint
        .as_ref()
        .and_then(|sprints| sprints.last())
        .and_then(|spr| spr.name.clone())
        .unwrap_or_default()
}

/// "EVB-123" -> "EVB", "CC-45" -> "CC"
pub fn extract_project_key(
    issue_key: &str,
) -> String {
    issue_key
        .split('-')
        .next()
        .unwrap_or(issue_key)
        .to_string()
}
