use crate::app::App;

/// Trigger characters that activate picker modes
const TRIGGER_CHARS: [char; 3] = ['@', '#', '/'];

impl App {
    /// Cancel picker and remove the trigger from input
    pub(crate) fn cancel_picker(&mut self) {
        self.remove_trigger_char();
        self.picker.deactivate();
    }

    pub(crate) fn remove_trigger_char(&mut self) {
        let pos = self.picker.trigger_position();
        let chr = self.input[pos..].chars().next();
        let is_trigger = chr
            .is_some_and(|chr| TRIGGER_CHARS.contains(&chr));
        if pos < self.input.len() && is_trigger {
            self.input.remove(pos);
            if self.cursor_position.get() > pos {
                self.cursor_position.set(pos);
            }
        }
    }
}
