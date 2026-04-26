use crossterm::event::KeyCode;

use crate::app::App;

impl App {
    /// Handle keys in RepoSelect mode
    pub(crate) fn handle_repo_select_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        match key {
            KeyCode::Esc => {
                self.picker.back_to_tools();
            }
            KeyCode::Enter => {
                self.confirm_repo_selection();
            }
            KeyCode::Backspace => {
                self.repo_select_backspace();
            }
            _ => self.repo_select_input(key),
        }
        false
    }

    /// Handle nav and text input for repo select
    fn repo_select_input(
        &mut self,
        key: KeyCode,
    ) {
        match key {
            KeyCode::Up => self.picker.move_up(),
            KeyCode::Down => {
                let count =
                    self.picker.filtered_repos().len();
                self.picker.move_down(count);
            }
            KeyCode::Char(chr) => {
                self.picker.push_query(chr);
                self.picker
                    .query.set_selected_index(0);
            }
            _ => {}
        }
    }

    /// Handle backspace in repo select filter
    fn repo_select_backspace(&mut self) {
        if self.picker.query().is_empty() {
            self.picker.back_to_tools();
        } else {
            self.picker.pop_query();
            self.picker
                .query.set_selected_index(0);
        }
    }

    /// Confirm selected repo and fetch tool data
    fn confirm_repo_selection(&mut self) {
        let tool = match self.picker.mode() {
            crate::picker::PickerMode::RepoSelect
                { tool } => *tool,
            _ => return,
        };
        let Some(repo) =
            self.picker.selected_repo().cloned()
        else {
            return;
        };
        self.fetch_with_repo(tool, &repo);
    }
}
