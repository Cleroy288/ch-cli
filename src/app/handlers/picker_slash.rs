use crossterm::event::KeyCode;

use crate::app::App;
use crate::picker::slash_items;

impl App {
    /// Handle keyboard events in SlashCommand or
    /// SlashArg mode.
    pub(crate) fn handle_slash_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        match key {
            KeyCode::Up => self.picker.move_up(),
            KeyCode::Down => {
                let count = self.slash_item_count();
                self.picker.move_down(count);
            }
            KeyCode::Char(chr) => {
                self.picker.push_query(chr);
                if self.slash_item_count() == 0 {
                    self.dismiss_slash_as_text();
                }
            }
            KeyCode::Backspace => {
                self.handle_slash_backspace();
            }
            KeyCode::Enter => self.select_slash_item(),
            KeyCode::Esc => {
                self.cancel_slash_or_back();
            }
            _ => {}
        }
        false
    }

    /// Handle backspace in slash picker
    fn handle_slash_backspace(&mut self) {
        if self.picker.query().is_empty() {
            self.cancel_slash_or_back();
        } else {
            self.picker.pop_query();
        }
    }

    /// Dismiss picker, merge query back into input
    fn dismiss_slash_as_text(&mut self) {
        let query = self.picker.query().to_string();
        self.input.push_str(&query);
        self.cursor_position.set(self.input.len());
        self.picker.deactivate();
    }

    fn slash_item_count(&self) -> usize {
        slash_items::all_display_items(
            self.picker.mode(),
            self.picker.query(),
            &self.skills,
        )
        .len()
    }
}
