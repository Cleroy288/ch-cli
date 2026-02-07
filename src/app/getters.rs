use crate::domain::FileReference;
use crate::message::ConversationHistory;
use crate::picker::Picker;

use super::App;

impl App {
    /// Get the current input text
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Get the cursor position (as raw usize for rendering)
    pub fn cursor_position(&self) -> usize {
        self.cursor_position.get()
    }

    /// Check if the app should quit
    pub fn should_quit(&self) -> bool {
        self.should_quit
    }

    /// Get a reference to the picker
    pub fn picker(&self) -> &Picker {
        &self.picker
    }

    /// Get file references
    pub fn file_references(&self) -> &[FileReference] {
        &self.file_references
    }

    /// Get conversation history
    pub fn history(&self) -> &ConversationHistory {
        &self.history
    }
}
