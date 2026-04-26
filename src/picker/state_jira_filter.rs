use crate::picker::mode::PickerMode;

use super::state::Picker;

impl Picker {
    pub fn jira_assignees(&self) -> &[String] {
        &self.jira_assignees
    }

    pub fn jira_assignee_filter(
        &self,
    ) -> Option<&str> {
        self.jira_assignee_filter.as_deref()
    }

    pub fn set_jira_assignee_filter(
        &mut self,
        filter: Option<String>,
    ) {
        self.jira_assignee_filter = filter;
    }

    /// Enter assignee filter sub-picker
    pub fn enter_assignee_filter(&mut self) {
        self.mode = PickerMode::JiraAssigneeFilter;
        self.query.clear();
    }

    /// Store fetched assignees and open picker
    pub fn activate_assignee_picker(
        &mut self,
        names: Vec<String>,
    ) {
        self.jira_assignees = names;
        self.mode = PickerMode::JiraAssigneeFilter;
        self.query.clear();
        self.tool_error = None;
    }
}
