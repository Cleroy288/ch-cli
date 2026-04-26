use crate::domain::jira_detail::{
    JiraComment, JiraIssueDetail,
};
use crate::service::tools::jira_adf;
use crate::service::tools::jira_extract;
use crate::service::tools::jira_types::{
    JiraCommentEntry, JiraFields, JiraIssue,
};

pub fn map_issue_detail(
    issue: &JiraIssue,
) -> JiraIssueDetail {
    let fields = &issue.fields;
    let description = extract_description(fields);
    let comments = extract_comments(fields);
    JiraIssueDetail {
        key: issue.key.clone(),
        summary: fields.summary.clone(),
        status: fields.status.name.clone(),
        priority: extract_priority(fields),
        issue_type: jira_extract::extract_type(
            fields,
        ),
        assignee: jira_extract::extract_assignee(
            fields,
        ),
        story_points: fields.story_points,
        description,
        comments,
    }
}

/// Extract description text from ADF
fn extract_description(
    fields: &JiraFields,
) -> String {
    fields
        .description
        .as_ref()
        .map(jira_adf::adf_to_text)
        .unwrap_or_default()
}

/// Extract priority name with default
fn extract_priority(
    fields: &JiraFields,
) -> String {
    fields
        .priority
        .as_ref()
        .map(|pri| pri.name.clone())
        .unwrap_or_else(|| "Medium".into())
}

fn extract_comments(
    fields: &JiraFields,
) -> Vec<JiraComment> {
    let Some(container) = &fields.comment else {
        return Vec::new();
    };
    let Some(entries) = &container.comments else {
        return Vec::new();
    };
    entries.iter().map(map_comment).collect()
}

fn map_comment(
    entry: &JiraCommentEntry,
) -> JiraComment {
    let author = entry
        .author
        .as_ref()
        .map(|who| who.display_name.clone())
        .unwrap_or_else(|| "Unknown".into());
    let body = entry
        .body
        .as_ref()
        .map(jira_adf::adf_to_text)
        .unwrap_or_default();
    let created = entry
        .created
        .clone()
        .unwrap_or_default();
    JiraComment { author, body, created }
}
