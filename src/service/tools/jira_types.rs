use serde::Deserialize;

pub const USER_JQL: &str =
    "assignee = currentUser() \
    ORDER BY updated DESC";

#[derive(Deserialize)]
pub struct JiraSearchResult {
    pub issues: Vec<JiraIssue>,
}

#[derive(Deserialize)]
pub struct JiraIssue {
    pub key: String,
    pub fields: JiraFields,
}

#[derive(Deserialize)]
pub struct JiraFields {
    pub summary: String,
    pub status: JiraStatus,
    #[serde(default)]
    pub priority: Option<JiraPriority>,
    #[serde(default)]
    pub issuetype: Option<JiraIssueType>,
    #[serde(default)]
    pub assignee: Option<JiraAssignee>,
    /// ADF JSON
    #[serde(default)]
    pub description: Option<serde_json::Value>,
    #[serde(
        default,
        rename = "customfield_10016"
    )]
    pub story_points: Option<f64>,
    #[serde(
        default,
        rename = "customfield_10021"
    )]
    pub sprint: Option<Vec<JiraSprintRaw>>,
    #[serde(default)]
    pub comment: Option<JiraCommentContainer>,
}

#[derive(Deserialize)]
pub struct JiraStatus {
    pub name: String,
    #[serde(
        default,
        rename = "statusCategory"
    )]
    pub status_category:
        Option<JiraStatusCategory>,
}

#[derive(Deserialize)]
pub struct JiraStatusCategory {
    pub name: String,
    /// Locale-independent key
    #[serde(default)]
    pub key: Option<String>,
}

#[derive(Deserialize)]
pub struct JiraPriority {
    pub name: String,
}

#[derive(Deserialize)]
pub struct JiraIssueType {
    pub name: String,
}

#[derive(Deserialize)]
pub struct JiraAssignee {
    #[serde(rename = "displayName")]
    pub display_name: String,
}

#[derive(Deserialize)]
pub struct JiraSprintRaw {
    #[serde(default)]
    pub name: Option<String>,
    /// active / closed / future
    #[serde(default)]
    pub state: Option<String>,
    #[serde(default, rename = "startDate")]
    pub start_date: Option<String>,
    #[serde(default, rename = "endDate")]
    pub end_date: Option<String>,
}

#[derive(Deserialize)]
pub struct JiraCommentContainer {
    #[serde(default)]
    pub total: Option<u32>,
    #[serde(default)]
    pub comments: Option<Vec<JiraCommentEntry>>,
}

#[derive(Deserialize)]
pub struct JiraCommentEntry {
    #[serde(default)]
    pub author: Option<JiraAssignee>,
    /// ADF JSON
    #[serde(default)]
    pub body: Option<serde_json::Value>,
    #[serde(default)]
    pub created: Option<String>,
}
