use super::mode::PickerMode;
use super::state::Picker;

impl Picker {
    pub fn activate_slash(&mut self, position: usize) {
        self.mode = PickerMode::SlashCommand;
        self.query.clear();
        self.trigger_position = position;
        self.symbol_browser = None;
    }

    /// Transition to slash argument mode
    pub fn activate_slash_arg(
        &mut self,
        command: &str,
    ) {
        self.mode = PickerMode::SlashArg {
            command: command.to_string(),
        };
        self.query.clear();
    }

    pub fn is_slash_mode(&self) -> bool {
        matches!(
            self.mode,
            PickerMode::SlashCommand
                | PickerMode::SlashArg { .. }
        )
    }

    /// Go back from SlashArg to SlashCommand
    pub fn back_to_slash_command(&mut self) {
        self.mode = PickerMode::SlashCommand;
        self.query.clear();
    }
}
