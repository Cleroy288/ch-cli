use crossterm::event::KeyCode;

use crate::app::App;
use crate::picker::tool_items;

impl App {
    /// Handle keyboard events in Tools menu mode
    pub(crate) fn handle_tools_key(
        &mut self,
        key: KeyCode,
    ) -> bool {
        let count = self.filtered_tool_count();
        match key {
            KeyCode::Up => self.picker.move_up(),
            KeyCode::Down => {
                self.picker.move_down(count);
            }
            KeyCode::Char(chr) => {
                self.picker.push_query(chr);
            }
            KeyCode::Backspace => {
                self.handle_tools_backspace();
            }
            KeyCode::Enter => {
                self.select_tool_item();
            }
            KeyCode::Esc => self.cancel_picker(),
            KeyCode::BackTab => {}
            _ => {}
        }
        false
    }

    /// Handle backspace: pop query or cancel
    fn handle_tools_backspace(&mut self) {
        if self.picker.query().is_empty() {
            self.cancel_picker();
        } else {
            self.picker.pop_query();
        }
    }

    fn filtered_tool_count(&self) -> usize {
        tool_items::filter_tools(
            self.picker.query(),
        )
        .len()
    }
}
