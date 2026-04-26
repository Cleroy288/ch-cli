use crate::app::App;
use crate::picker::mode::PickerMode;

impl App {
    /// Cancel slash picker entirely (Esc)
    pub(super) fn cancel_slash(&mut self) {
        self.remove_trigger_char();
        self.picker.deactivate();
    }

    /// Backspace on empty query: back or cancel
    ///
    /// SlashArg → go back to SlashCommand with "/"
    /// SlashCommand → cancel and remove /
    pub(super) fn cancel_slash_or_back(&mut self) {
        match self.picker.mode() {
            PickerMode::SlashArg { .. } => {
                self.back_to_slash_command();
            }
            _ => self.cancel_slash(),
        }
    }

    /// Return to SlashCommand mode from SlashArg
    fn back_to_slash_command(&mut self) {
        self.input.clear();
        self.input.push('/');
        self.cursor_position.set(self.input.len());
        self.picker.back_to_slash_command();
    }
}
