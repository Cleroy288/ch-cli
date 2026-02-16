use crossterm::event::KeyCode;

use crate::app::App;

impl App {
    /// Handle keys in Browse mode (filesystem).
    ///
    /// Enter on dir → drill into. Enter on file →
    /// select. Backspace → go to parent dir.
    pub(crate) fn handle_browse_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        match key {
            KeyCode::F(5) => {},
            KeyCode::Up => self.picker.move_up(),
            KeyCode::Down => {
                let count = self.picker.get_results().len();
                self.picker.move_down(count);
            }
            KeyCode::Enter => {
                self.handle_browse_enter();
            }
            KeyCode::Esc => self.cancel_picker(),
            KeyCode::Backspace => {
                self.handle_browse_backspace();
            }
            KeyCode::Char(chr) => {
                self.picker.push_query(chr);
            }
            _ => {}
        }
        false
    }

    /// Handle Enter in browse mode.
    ///
    /// If selected entry is a dir, drill into it.
    /// If a Rust file, auto-open symbol picker.
    /// Otherwise, insert reference and deactivate.
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
        let is_rust = path.ends_with(".rs");
        let name = entry.name_only();
        self.insert_selected_path(path, name, false);

        if is_rust {
            self.auto_open_symbol_picker();
        } else {
            self.picker.deactivate();
        }
    }

    /// Auto-insert ( and open symbol picker.
    ///
    /// Called after selecting a Rust file in browse
    /// mode. If symbol picker fails to activate,
    /// cleans up the ( and deactivates the picker.
    fn auto_open_symbol_picker(&mut self) {
        let pos = self.cursor_position.get();
        self.input.insert(pos, '(');
        self.cursor_position.move_right(
            self.input.len(),
        );

        self.try_open_symbol_picker();

        if !self.picker.is_symbol_mode() {
            self.remove_trailing_paren();
            self.picker.deactivate();
        }
    }

    /// Remove the last ( if symbol picker failed
    fn remove_trailing_paren(&mut self) {
        let pos = self.cursor_position.get();
        if pos > 0 {
            self.input.remove(pos - 1);
            self.cursor_position.move_left();
        }
    }

    /// Backspace in browse: go to parent dir or cancel
    fn handle_browse_backspace(&mut self) {
        if !self.picker.query().is_empty() {
            self.picker.pop_query();
            return;
        }
        self.cancel_picker();
    }
}
