use crossterm::event::KeyCode;

use crate::app::App;
use crate::domain::tool_ref::ToolKind;

impl App {
    /// Handle keys in JiraBoardSelect mode
    pub(crate) fn handle_board_select_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        let count =
            self.filtered_project_keys().len();
        match key {
            KeyCode::Up => self.picker.move_up(),
            KeyCode::Down => {
                self.picker.move_down(count);
            }
            KeyCode::Enter => self.select_project(),
            KeyCode::Esc => {
                self.cancel_space_select();
            }
            KeyCode::Backspace => {
                self.space_select_backspace();
            }
            KeyCode::Char(chr) => {
                self.picker.push_query(chr);
            }
            _ => {}
        }
        false
    }

    pub(crate) fn filtered_project_keys(
        &self,
    ) -> Vec<&str> {
        let query =
            self.picker.query().to_lowercase();
        self.picker
            .jira_project_keys()
            .iter()
            .filter(|key| {
                query.is_empty()
                    || key
                        .to_lowercase()
                        .contains(&query)
            })
            .map(|key| key.as_str())
            .collect()
    }

    /// Select a project and show its tickets
    fn select_project(&mut self) {
        let list = self.filtered_project_keys();
        let idx = self.picker.selected_index();
        let Some(key) = list.get(idx) else {
            return;
        };
        let key = key.to_string();
        self.picker
            .set_jira_selected_project(Some(key));
        self.picker
            .enter_tool_results(ToolKind::Jira);
    }

    /// Cancel: back to assignee filter
    fn cancel_space_select(&mut self) {
        self.picker.clear_jira_projects();
        self.picker.enter_assignee_filter();
    }

    /// Backspace: pop query or cancel
    fn space_select_backspace(&mut self) {
        if self.picker.query().is_empty() {
            self.cancel_space_select();
        } else {
            self.picker.pop_query();
        }
    }
}
