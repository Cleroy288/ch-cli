use std::fmt;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum SprintState {
    Active,
    Closed,
    #[default]
    Future,
}

impl fmt::Display for SprintState {
    fn fmt(
        &self,
        f: &mut fmt::Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Active => f.write_str("active"),
            Self::Closed => f.write_str("closed"),
            Self::Future => f.write_str("future"),
        }
    }
}

impl SprintState {
    pub fn from_str_lossy(s: &str) -> Self {
        match s {
            "active" => Self::Active,
            "closed" => Self::Closed,
            _ => Self::Future,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct JiraSprint {
    pub name: String,
    pub state: SprintState,
    pub start_date: String,
    pub end_date: String,
}

#[derive(Debug, Clone)]
pub struct JiraTicketDetail {
    pub key: String,
    pub summary: String,
    pub status: String,
    pub status_category: String,
    pub priority: String,
    pub issue_type: String,
    pub assignee: String,
    pub story_points: Option<f64>,
    pub sprint_name: String,
    pub project_key: String,
}

#[derive(Debug, Clone, Default)]
pub struct JiraBoardData {
    pub sprint: Option<JiraSprint>,
    pub tickets: Vec<JiraTicketDetail>,
    pub total_points: f64,
}
