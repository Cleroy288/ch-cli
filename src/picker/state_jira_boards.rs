use crate::picker::mode::PickerMode;

use super::state::Picker;

impl Picker {
    /// Store keys and enter board select mode
    pub fn enter_space_select(
        &mut self,
        keys: Vec<String>,
    ) {
        self.jira_project_keys = keys;
        self.mode = PickerMode::JiraBoardSelect;
        self.query.clear();
    }

    pub fn jira_project_keys(&self) -> &[String] {
        &self.jira_project_keys
    }

    pub fn set_jira_selected_project(
        &mut self,
        key: Option<String>,
    ) {
        self.jira_selected_project = key;
    }

    pub fn jira_selected_project(
        &self,
    ) -> Option<&str> {
        self.jira_selected_project.as_deref()
    }

    pub fn clear_jira_projects(&mut self) {
        self.jira_project_keys.clear();
        self.jira_selected_project = None;
    }

    /// Return to board select (keeps existing keys)
    pub fn back_to_board_select(&mut self) {
        self.jira_selected_project = None;
        self.mode = PickerMode::JiraBoardSelect;
        self.query.clear();
    }
}
