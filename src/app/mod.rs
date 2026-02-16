use crossterm::event::{KeyCode, KeyModifiers};

use std::collections::HashSet;
use std::sync::mpsc;

use crate::domain::claude::ClaudeResponse;
use crate::domain::errors::ClaudeError;
use crate::domain::memory::UserInput;
use crate::domain::memory_helpers;
use crate::domain::{
	CursorPosition, FileReference, SymbolSelector,
};
use crate::fs::FileCache;
use crate::message::ConversationHistory;
use crate::picker::Picker;
use crate::retrieval::daemon::protocol::DocEntryResponse;

pub mod claude_poll;
pub mod claude_request;
pub mod doc_progress;
pub mod doc_progress_poll;
pub mod doc_preview_poll;
mod getters;
#[doc(hidden)]
pub mod handlers;
pub mod memory_save;
#[doc(hidden)]
pub mod parser;

/// Main application state.
///
/// Manages the input text, cursor position,
/// file/folder references, symbol selectors,
/// picker state, and conversation history.
pub struct App {
	/// The current input text
	input: String,
	/// Current cursor position
	cursor_position: CursorPosition,
	/// Flag to indicate the app should quit
	should_quit: bool,
	/// File/folder picker
	picker: Picker,
	/// Track file/folder references in the input
	file_references: Vec<FileReference>,
	/// Track symbol selectors in the input
	symbol_selectors: Vec<SymbolSelector>,
	/// Conversation history
	history: ConversationHistory,
	/// Background doc generation progress
	doc_progress: doc_progress::DocProgressState,
	/// Doc preview for last selected symbol
	doc_preview: Option<DocEntryResponse>,
	/// Project root path for daemon queries
	project_path: String,
	/// Transient status message (cleared on next key)
	status_message: Option<String>,
	/// True when doc fetch completed with no result
	doc_fetch_no_result: bool,
	/// Receiver for background doc names fetch
	doc_names_rx: Option<mpsc::Receiver<HashSet<String>>>,
	/// True = use --continue on next Claude request
	pub(crate) continue_session: bool,
	/// Receiver for background Claude CLI response
	#[allow(clippy::type_complexity)]
	claude_rx: Option<
		mpsc::Receiver<Result<ClaudeResponse, ClaudeError>>,
	>,
	/// Last Claude response for display
	last_claude_response: Option<ClaudeResponse>,
	/// Vertical scroll offset for the output panel
	scroll_offset: u16,
	/// Session ID for memory auto-save
	memory_session_id: String,
	/// Pending user input awaiting Claude response
	pending_user_input: Option<UserInput>,
}

/// Resolve the current working directory as a string
fn current_project_path() -> String {
	std::env::current_dir()
		.unwrap_or_default()
		.display()
		.to_string()
}

impl App {
	/// Create a new App with a shared file cache
	pub fn new(file_cache: FileCache) -> Self {
		Self {
			input: String::new(),
			cursor_position: CursorPosition::new(),
			should_quit: false,
			picker: Picker::new(file_cache),
			file_references: Vec::new(),
			symbol_selectors: Vec::new(),
			history: ConversationHistory::new(),
			doc_progress:
				doc_progress::DocProgressState::new(),
			doc_preview: None,
			project_path: current_project_path(),
			status_message: None,
			doc_fetch_no_result: false,
			doc_names_rx: None,
			continue_session: true,
			claude_rx: None,
			last_claude_response: None,
			scroll_offset: 0,
			memory_session_id:
				memory_helpers::new_session_id(),
			pending_user_input: None,
		}
	}

	/// Sync picker file cache from watcher updates
	pub fn sync_file_cache(&mut self) {
		self.picker.sync_cache();
	}

	/// Handle keyboard input and update app state.
	///
	/// Routes keyboard events to appropriate handlers
	/// based on picker state.
	/// Returns true if the app should quit.
	pub fn handle_key(
		&mut self,
		key: KeyCode,
		modifiers: KeyModifiers,
	) -> bool {
		self.status_message = None;
		let is_ctrl_c = key == KeyCode::Char('c')
			&& modifiers.contains(KeyModifiers::CONTROL);
		if is_ctrl_c {
			self.should_quit = true;
			return true;
		}

		if self.picker.is_active() {
			return self.handle_picker_key(key);
		}

		self.handle_input_key(key)
	}
}

impl Default for App {
	fn default() -> Self {
		Self::new(FileCache::empty())
	}
}
