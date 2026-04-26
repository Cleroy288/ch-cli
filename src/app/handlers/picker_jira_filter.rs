use crossterm::event::KeyCode;

use crate::app::App;
use crate::domain::tool_ref::ToolKind;

impl App {
    pub(crate) fn handle_jira_filter_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        let count = self.assignee_list_count();
        match key {
            KeyCode::Up => self.picker.move_up(),
            KeyCode::Down => {
                self.picker.move_down(count);
            }
            KeyCode::Char(chr) => {
                self.picker.push_query(chr);
            }
            KeyCode::Backspace => {
                self.jira_filter_backspace();
            }
            KeyCode::Enter => {
                self.select_assignee_filter();
            }
            KeyCode::Esc => {
                self.cancel_assignee_filter();
            }
            _ => {}
        }
        false
    }

    fn jira_filter_backspace(&mut self) {
        if self.picker.query().is_empty() {
            self.cancel_assignee_filter();
        } else {
            self.picker.pop_query();
        }
    }

    fn select_assignee_filter(&mut self) {
        let list = self.filtered_assignee_list();
        let idx = self.picker.selected_index();
        let Some(name) = list.get(idx) else {
            return;
        };
        self.picker.set_jira_assignee_filter(
            Some(name.clone()),
        );
        self.fetch_jira_bg();
    }

    fn cancel_assignee_filter(&mut self) {
        if self.picker.jira_board().is_some() {
            self.picker
                .enter_tool_results(ToolKind::Jira);
        } else {
            self.picker.back_to_tools();
        }
    }
}
