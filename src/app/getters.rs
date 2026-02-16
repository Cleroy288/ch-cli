use crate::app::doc_progress::DocProgressState;
use crate::domain::claude::ClaudeResponse;
use crate::domain::memory::UserInput;
use crate::domain::{FileReference, SymbolSelector};
use crate::message::ConversationHistory;
use crate::picker::Picker;
use crate::retrieval::daemon::protocol::DocEntryResponse;

use super::App;

impl App {
    /// Get the current input text
    pub fn input(&self) -> &str {
        &self.input
    }

    /// Get the cursor position
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

    /// Get symbol selectors
    pub fn symbol_selectors(
        &self,
    ) -> &[SymbolSelector] {
        &self.symbol_selectors
    }

    /// Get conversation history
    pub fn history(&self) -> &ConversationHistory {
        &self.history
    }

    /// Get doc generation progress state
    pub fn doc_progress(&self) -> &DocProgressState {
        &self.doc_progress
    }

    /// Get current doc preview (if any)
    pub fn doc_preview(
        &self,
    ) -> Option<&DocEntryResponse> {
        self.doc_preview.as_ref()
    }

    /// Get mutable doc progress for polling updates
    pub fn doc_progress_mut(
        &mut self,
    ) -> &mut DocProgressState {
        &mut self.doc_progress
    }

    /// Get the transient status message
    pub fn status_message(&self) -> Option<&str> {
        self.status_message.as_deref()
    }

    /// True when doc fetch completed with no result
    pub fn doc_fetch_no_result(&self) -> bool {
        self.doc_fetch_no_result
    }

    /// Set status message (for testing)
    pub fn set_status_message(
        &mut self,
        msg: Option<String>,
    ) {
        self.status_message = msg;
    }

    /// Get last Claude response (if any)
    pub fn last_claude_response(
        &self,
    ) -> Option<&ClaudeResponse> {
        self.last_claude_response.as_ref()
    }

    /// True when waiting for Claude CLI response
    pub fn is_claude_loading(&self) -> bool {
        self.claude_rx.is_some()
    }

    /// Get the output panel scroll offset
    pub fn scroll_offset(&self) -> u16 {
        self.scroll_offset
    }

    /// Get the memory session ID
    pub fn memory_session_id(&self) -> &str {
        &self.memory_session_id
    }

    /// Get the pending user input (if any)
    pub fn pending_user_input(
        &self,
    ) -> Option<&UserInput> {
        self.pending_user_input.as_ref()
    }

    /// Set Claude receiver (for testing)
    #[allow(clippy::type_complexity)]
    pub fn set_claude_rx(
        &mut self,
        recv: Option<
            std::sync::mpsc::Receiver<
                Result<
                    ClaudeResponse,
                    crate::domain::errors::ClaudeError,
                >,
            >,
        >,
    ) {
        self.claude_rx = recv;
    }
}
