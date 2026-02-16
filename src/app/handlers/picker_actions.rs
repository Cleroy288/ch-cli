use crate::app::App;

/// Trigger characters that activate picker modes
const TRIGGER_CHARS: [char; 2] = ['@', '#'];

impl App {
    /// Cancel picker and remove the trigger from input
    pub(crate) fn cancel_picker(&mut self) {
        self.remove_trigger_char();
        self.picker.deactivate();
    }

    /// Remove the trigger character (@ or #) from input
    pub(crate) fn remove_trigger_char(&mut self) {
        let pos = self.picker.trigger_position();
        let chr = self.input.chars().nth(pos);
        let is_trigger = chr
            .is_some_and(|c| TRIGGER_CHARS.contains(&c));
        if pos < self.input.len() && is_trigger {
            self.input.remove(pos);
            if self.cursor_position.get() > pos {
                self.cursor_position.move_left();
            }
        }
    }
}
