use crossterm::event::KeyCode;

use crate::app::App;
use super::picker_symbol_actions::has_parser_support;

impl App {
    pub(crate) fn handle_browse_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        match key {
            KeyCode::F(5) => {}
            KeyCode::Up => self.picker.move_up(),
            KeyCode::Down => {
                let count =
                    self.picker.get_results().len();
                self.picker.move_down(count);
            }
            KeyCode::Enter => {
                self.handle_browse_enter()
            }
            KeyCode::Tab => self.handle_browse_tab(),
            KeyCode::Esc => self.cancel_picker(),
            KeyCode::Backspace => {
                self.handle_browse_backspace()
            }
            KeyCode::Char(chr) => {
                self.picker.push_query(chr)
            }
            KeyCode::BackTab => {}
            _ => {}
        }
        false
    }

    /// Dir → drill in, supported file → symbol picker.
    fn handle_browse_enter(&mut self) {
        let entry =
            self.picker.get_selected_entry().cloned();
        let Some(entry) = entry else { return };

        if entry.is_dir {
            let dir_path = entry.path.clone();
            self.picker.browse_into(dir_path);
            return;
        }

        let path = entry.path_string();
        let parseable = has_parser_support(&path);
        let name = entry.name_only();
        self.insert_selected_path(path, name, false);

        if parseable {
            self.auto_open_symbol_picker();
        } else {
            self.picker.deactivate();
        }
    }

    fn handle_browse_tab(&mut self) {
        let entry =
            self.picker.get_selected_entry().cloned();
        let Some(entry) = entry else { return };

        let path = entry.path_string();
        let name = if entry.is_dir {
            format!("{}/", entry.name_only())
        } else {
            entry.name_only()
        };
        self.insert_selected_path(
            path, name, entry.is_dir,
        );
        self.picker.deactivate();
    }

    /// Insert `(`, try symbol picker, undo on failure.
    fn auto_open_symbol_picker(&mut self) {
        let pos = self.cursor_position.get();
        self.input.insert(pos, '(');
        self.cursor_position.move_right(
            &self.input,
        );
        self.try_open_symbol_picker();
        if self.picker.is_symbol_mode() {
            return;
        }
        // Undo the `(` insertion
        self.input.remove(pos);
        self.cursor_position.set(pos);
        self.picker.deactivate();
    }

    fn handle_browse_backspace(&mut self) {
        if !self.picker.query().is_empty() {
            self.picker.pop_query();
            return;
        }
        self.cancel_picker();
    }
}
