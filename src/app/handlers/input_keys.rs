use crossterm::event::{KeyCode, KeyModifiers};
use std::time::Instant;

use crate::app::App;
use crate::ui::strings::tui_labels;

const DOUBLE_ESC_MS: u128 = 1500;

impl App {
    /// Returns true if the app should quit.
    pub(crate) fn handle_input_key(
        &mut self,
        key: KeyCode,
        mods: KeyModifiers,
    ) -> bool {
        if is_ctrl_char(key, mods, 'e') {
            self.start_enhance();
            return false;
        }
        if is_ctrl_char(key, mods, 'r') {
            self.activate_review_mode();
            return false;
        }
        if is_shift_enter(key, mods) {
            self.insert_newline();
            return false;
        }
        self.dispatch_input_key(key, mods)
    }

    fn dispatch_input_key(
        &mut self,
        key: KeyCode,
        mods: KeyModifiers,
    ) -> bool {
        if key != KeyCode::Up
            && key != KeyCode::Down
        {
            self.history_index = None;
        }
        match key {
            KeyCode::Char(chr) => {
                self.handle_char_input(chr)
            }
            KeyCode::Backspace => {
                self.handle_backspace()
            }
            KeyCode::Delete => self.handle_delete(),
            KeyCode::Left => {
                self.handle_cursor_left()
            }
            KeyCode::Right => {
                self.handle_cursor_right()
            }
            KeyCode::Up => self.handle_up(),
            KeyCode::Down => self.handle_down(),
            KeyCode::Home => {
                self.handle_home_key(mods)
            }
            KeyCode::End => {
                self.handle_end_key(mods)
            }
            KeyCode::Enter => self.handle_enter(),
            KeyCode::Esc => {
                return self.handle_esc()
            }
            _ => {}
        }
        false
    }

    /// Up: move up in multiline, or history.
    fn handle_up(&mut self) {
        if self.input.contains('\n')
            && !self.cursor_on_first_line()
        {
            self.cursor_move_up();
        } else {
            self.history_prev();
        }
    }

    /// Down: move down in multiline, or history.
    fn handle_down(&mut self) {
        if self.input.contains('\n')
            && !self.cursor_on_last_line()
        {
            self.cursor_move_down();
        } else {
            self.history_next();
        }
    }

    /// Double-Esc within 1.5s to quit.
    fn handle_esc(&mut self) -> bool {
        if let Some(ts) = self.esc_pressed_at {
            let elapsed = ts.elapsed().as_millis();
            if elapsed <= DOUBLE_ESC_MS {
                self.should_quit = true;
                return true;
            }
        }
        self.esc_pressed_at = Some(Instant::now());
        self.status_message = Some(
            tui_labels::QUIT_HINT.to_string(),
        );
        false
    }
}

fn is_ctrl_char(
    key: KeyCode,
    mods: KeyModifiers,
    c: char,
) -> bool {
    key == KeyCode::Char(c)
        && mods.contains(KeyModifiers::CONTROL)
}

/// Shift+Enter or Char('\n') with no modifiers.
fn is_shift_enter(
    key: KeyCode,
    mods: KeyModifiers,
) -> bool {
    if key == KeyCode::Enter
        && mods.contains(KeyModifiers::SHIFT)
    {
        return true;
    }
    // Some terminals send Char('\n') for Shift+Enter
    key == KeyCode::Char('\n')
}
