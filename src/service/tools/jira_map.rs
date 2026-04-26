use crate::domain::jira::{
    JiraBoardData, JiraSprint,
    JiraTicketDetail, SprintState,
};
use crate::domain::ToDomain;

use super::jira_extract;
use super::jira_types::{
    JiraIssue, JiraSprintRaw,
};

/// Backward-compatible wrapper.
pub fn map_board(
    issues: Vec<JiraIssue>,
) -> JiraBoardData {
    issues.to_domain()
}

fn extract_active_sprint(
    issues: &[JiraIssue],
) -> Option<JiraSprint> {
    issues
        .iter()
        .filter_map(|i| i.fields.sprint.as_ref())
        .flat_map(|sprints| sprints.iter())
        .find(|spr| {
            spr.state.as_deref()
                == Some("active")
        })
        .map(|spr| spr.to_domain())
}

impl ToDomain<JiraTicketDetail> for JiraIssue {
    fn to_domain(&self) -> JiraTicketDetail {
        let fields = &self.fields;
        let category = fields
            .status
            .status_category
            .as_ref()
            .and_then(|cat| cat.key.clone())
            .unwrap_or_else(|| "new".into());
        let priority = fields
            .priority
            .as_ref()
            .map(|pri| pri.name.clone())
            .unwrap_or_else(|| "Medium".into());
        JiraTicketDetail {
            key: self.key.clone(),
            summary: fields.summary.clone(),
            status: fields.status.name.clone(),
            status_category: category,
            priority,
            issue_type:
                jira_extract::extract_type(fields),
            assignee:
                jira_extract::extract_assignee(
                    fields,
                ),
            story_points: fields.story_points,
            sprint_name:
                jira_extract::extract_sprint_name(
                    fields,
                ),
            project_key:
                jira_extract::extract_project_key(
                    &self.key,
                ),
        }
    }
}

impl ToDomain<JiraSprint> for JiraSprintRaw {
    fn to_domain(&self) -> JiraSprint {
        let state_str =
            self.state.as_deref().unwrap_or("");
        JiraSprint {
            name: self
                .name
                .clone()
                .unwrap_or_default(),
            state: SprintState::from_str_lossy(
                state_str,
            ),
            start_date: self
                .start_date
                .clone()
                .unwrap_or_default(),
            end_date: self
                .end_date
                .clone()
                .unwrap_or_default(),
        }
    }
}

impl ToDomain<JiraBoardData> for Vec<JiraIssue> {
    fn to_domain(&self) -> JiraBoardData {
        let sprint =
            extract_active_sprint(self);
        let tickets: Vec<JiraTicketDetail> = self
            .iter()
            .map(|i| i.to_domain())
            .collect();
        let total_points = tickets
            .iter()
            .filter_map(|t| t.story_points)
            .sum();
        JiraBoardData {
            sprint,
            tickets,
            total_points,
        }
    }
}
