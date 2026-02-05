use crossterm::event::{KeyCode, KeyModifiers};

use crate::domain::{CursorPosition, FileReference};
use crate::message::ConversationHistory;
use crate::picker::Picker;

mod handlers;
mod parser;

/// Main application state.
///
/// Manages the input text, cursor position, file/folder references,
/// picker state, and conversation history.
pub struct App {
    /// The current input text
    input: String,
    /// Current cursor position (using NewType for type safety)
    cursor_position: CursorPosition,
    /// Flag to indicate the app should quit
    should_quit: bool,
    /// File/folder picker
    picker: Picker,
    /// Track file/folder references in the input
    file_references: Vec<FileReference>,
    /// Conversation history
    history: ConversationHistory,
}

impl App {
    /// Create a new App instance with default values
    pub fn new() -> Self {
        Self {
            input: String::new(),
            cursor_position: CursorPosition::new(),
            should_quit: false,
            picker: Picker::new(),
            file_references: Vec::new(),
            history: ConversationHistory::new(),
        }
    }

    /// Handle keyboard input and update app state.
    ///
    /// Routes keyboard events to appropriate handlers based on picker state.
    /// Uses guard clauses to avoid deep nesting.
    ///
    /// Returns true if the app should quit.
    pub fn handle_key(&mut self, key: KeyCode, modifiers: KeyModifiers) -> bool {
        // Guard clause: Handle Ctrl+C immediately
        if key == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL) {
            self.should_quit = true;
            return true;
        }

        // Guard clause: Route to picker handler if picker is active
        if self.picker.is_active() {
            return self.handle_picker_key(key);
        }

        // Handle regular input
        self.handle_input_key(key)
    }

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

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}
