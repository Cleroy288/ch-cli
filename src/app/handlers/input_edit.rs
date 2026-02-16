//! Text editing handlers for home, end, enter,
//! and file reference maintenance.

use crate::app::claude_request::spawn_claude_request;
use crate::app::handlers::symbol_resolver;
use crate::app::memory_save;
use crate::app::parser;
use crate::app::App;
use crate::domain::memory_helpers;
use crate::message::user_message::UserMessage;

/// Slash command to start a new conversation
const CMD_NEW: &str = "/new";

impl App {
    /// Jump cursor to start
    pub(crate) fn handle_home(&mut self) {
        self.cursor_position.jump_to_start();
    }

    /// Jump cursor to end
    pub(crate) fn handle_end(&mut self) {
        self.cursor_position.jump_to_end(
            self.input.len(),
        );
    }

    /// Handle enter key - parse, store, send
    pub(crate) fn handle_enter(&mut self) {
        if self.handle_slash_command() {
            return;
        }
        if let Some(msg) = self.parse_message() {
            self.send_and_store(msg);
        }
        self.clear_input_state();
    }

    /// Parse input into a UserMessage
    fn parse_message(&self) -> Option<UserMessage> {
        parser::parse_input_to_message(
            self.input.clone(),
            &self.file_references,
            &self.symbol_selectors,
        )
    }

    /// Resolve, capture, send, store
    fn send_and_store(
        &mut self,
        mut message: UserMessage,
    ) {
        symbol_resolver::resolve_symbols(
            &mut message.segments,
            &self.symbol_selectors,
        );
        self.pending_user_input = Some(
            memory_save::extract_user_input(
                &self.input, &message.segments,
            ),
        );
        spawn_claude_request(
            self, &message.segments,
        );
        self.history.add_message(message);
    }

    /// Handle slash commands (e.g. /new).
    /// Returns true if a command was handled.
    fn handle_slash_command(&mut self) -> bool {
        let trimmed = self.input.trim();
        if trimmed != CMD_NEW {
            return false;
        }
        self.continue_session = false;
        self.last_claude_response = None;
        self.memory_session_id =
            memory_helpers::new_session_id();
        self.pending_user_input = None;
        self.set_status_message(Some(
            "New conversation started".to_string(),
        ));
        self.clear_input_state();
        true
    }

    /// Reset input, cursor, refs after enter
    fn clear_input_state(&mut self) {
        self.input.clear();
        self.cursor_position.jump_to_start();
        self.file_references.clear();
        self.symbol_selectors.clear();
        self.doc_preview = None;
        self.doc_fetch_no_result = false;
    }

    /// Update file references when input changes.
    pub(crate) fn update_file_references(
        &mut self,
    ) {
        let input_len = self.input.len();
        self.file_references
            .retain(|fref| fref.end <= input_len);
        self.symbol_selectors
            .retain(|sel| sel.end <= input_len);
    }
}
