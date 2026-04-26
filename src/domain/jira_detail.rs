#[derive(Debug, Clone)]
pub struct JiraIssueDetail {
    pub key: String,
    pub summary: String,
    pub status: String,
    pub priority: String,
    pub issue_type: String,
    pub assignee: String,
    pub story_points: Option<f64>,
    pub description: String,
    pub comments: Vec<JiraComment>,
}

#[derive(Debug, Clone)]
pub struct JiraComment {
    pub author: String,
    pub body: String,
    pub created: String,
}
