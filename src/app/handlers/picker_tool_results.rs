use crossterm::event::KeyCode;

use crate::app::App;

impl App {
    /// Handle keys in ToolResults/ToolLoading mode
    pub(crate) fn handle_tool_results_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        let count = self.filtered_results_count();
        match key {
            KeyCode::Up => self.picker.move_up(),
            KeyCode::Down => {
                self.picker.move_down(count);
            }
            KeyCode::Char(chr) => {
                self.picker.push_query(chr);
            }
            KeyCode::Backspace => {
                self.tool_results_backspace();
            }
            KeyCode::Enter => {
                self.handle_tool_enter();
            }
            KeyCode::Esc => self.tool_results_esc(),
            _ => {}
        }
        false
    }

    /// Enter: filter, detail, or select result
    fn handle_tool_enter(&mut self) {
        if self.is_jira_filter_row() {
            self.picker.enter_assignee_filter();
        } else if self.is_jira_board_active() {
            self.open_jira_detail();
        } else {
            self.select_tool_result();
        }
    }

    /// Fetch detail for selected Jira ticket
    fn open_jira_detail(&mut self) {
        let Some(key) =
            self.selected_jira_key()
        else {
            return;
        };
        self.fetch_jira_detail_bg(key);
    }

    /// Esc: back to space picker or tools.
    ///
    /// If multiple projects exist, go back to
    /// the space picker. Otherwise back to tools.
    fn tool_results_esc(&mut self) {
        if !self.is_jira_board_active() {
            self.picker.back_to_tools();
            return;
        }
        let multi = self
            .picker
            .jira_project_keys()
            .len() > 1;
        if multi {
            self.picker.back_to_board_select();
        } else {
            self.picker.back_to_tools();
        }
    }

    /// Backspace: pop query or go back to tools
    fn tool_results_backspace(&mut self) {
        if self.picker.query().is_empty() {
            self.tool_results_esc();
        } else {
            self.picker.pop_query();
        }
    }
}
