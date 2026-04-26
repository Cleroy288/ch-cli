use std::cell::{Cell, RefCell};
use std::sync::{mpsc, Arc};
use std::time::Instant;

use ratatui::layout::Rect;

use crate::domain::agent_suggest::AgentSuggestion;
use crate::domain::claude::{
	ClaudeResponse, StreamChunk, ToolActivity,
};
use crate::domain::errors::{
	AtlassianError, BackendResult,
	GitResult,
};
use crate::domain::git_graph::GitHistory;
use crate::domain::memory::UserInput;
use crate::domain::preflight::PreFlightData;
use crate::domain::prompt_history::PromptHistory;
use crate::domain::review::BlockIdx;
use crate::domain::skill::SkillEntry;
use crate::domain::tool_ref::{
	ToolFetchResult, ToolReference,
};
use crate::domain::{
	CursorPosition, FileReference, SymbolSelector,
};
use crate::fs::FileCache;
use crate::message::ConversationHistory;
use crate::picker::Picker;
use crate::picker::mcp_display::McpDisplayItem;
use crate::review::InlineBlocks;
use crate::service::backend::CliBackend;
use crate::service::git::GitWatchHandle;
use crate::service::claude::model;
use crate::service::prompt_history_io;
use crate::service::skills;
use super::handlers::input_paste::PasteBlock;
use crate::service::memory::id_gen;

/// Main application state.
pub struct App {
	/// Active CLI backend (Claude, Gemini, ...)
	pub(crate) backend: Arc<dyn CliBackend>,
	pub(super) input: String,
	pub(super) cursor_position: CursorPosition,
	pub(super) should_quit: bool,
	pub(super) picker: Picker,
	pub(super) file_references: Vec<FileReference>,
	pub(super) symbol_selectors: Vec<SymbolSelector>,
	pub(super) history: ConversationHistory,
	pub(super) status_message: Option<String>,
	/// None = new session
	pub(crate) claude_session_id: Option<String>,
	pub(super) claude_rx:
		Option<mpsc::Receiver<StreamChunk>>,
	pub(super) last_claude_response:
		Option<ClaudeResponse>,
	/// Partial text accumulated during streaming
	pub(crate) streaming_text: String,
	pub(crate) tool_status: Option<ToolActivity>,
	pub(super) scroll_offset: u16,
	pub(super) memory_session_id: String,
	pub(super) pending_user_input: Option<UserInput>,
	/// haiku / sonnet / opus
	pub(super) model_name: String,
	/// low / medium / high / max
	pub(super) effort_level: String,
	#[allow(clippy::type_complexity)]
	pub(super) tool_rx: Option<
		mpsc::Receiver<
			Result<ToolFetchResult, AtlassianError>,
		>,
	>,
	pub(super) tool_references: Vec<ToolReference>,
	pub mcp_discovery_rx: Option<
		mpsc::Receiver<Vec<McpDisplayItem>>,
	>,
	pub(super) project_root: String,
	/// Placeholder -> real pasted content
	pub(crate) paste_blocks: Vec<PasteBlock>,
	/// Double-Esc to quit
	pub(super) esc_pressed_at: Option<Instant>,
	/// Double Ctrl+C to quit
	pub(super) ctrl_c_pressed_at: Option<Instant>,
	pub watcher_msg: Option<
		crate::startup::watcher::WatcherMsg,
	>,
	pub(crate) inline_blocks: Option<InlineBlocks>,
	/// Output panel Rect for click hit-testing
	pub(crate) output_area: Cell<Rect>,
	/// Cached screen-Y ranges of code blocks
	pub(crate) block_ranges: RefCell<Vec<(u16, u16)>>,
	/// Block index awaiting a question answer
	pub(crate) pending_block_question:
		Option<BlockIdx>,
	/// History browse index (0 = most recent)
	pub(super) history_index: Option<usize>,
	/// Input saved before browsing history
	pub(super) saved_input: String,
	/// Persistent prompt history (per-project)
	pub(super) prompt_history: PromptHistory,
	/// Receiver for enhance prompt result
	pub(super) enhance_rx:
		Option<mpsc::Receiver<BackendResult<String>>>,
	/// Original input before enhancement
	pub(super) pre_enhance_input: Option<String>,
	/// Agent lines stripped before enhance
	pub(super) pre_enhance_agents: Option<String>,
	/// Pre-flight panel data (agents detected)
	pub(super) preflight: Option<PreFlightData>,
	/// Auto-suggested agents from keyword detection
	pub(super) agent_suggestions:
		Vec<AgentSuggestion>,
	/// Claude skills from ~/.claude/commands/
	pub(crate) skills: Vec<SkillEntry>,
	/// Receiver for background git log fetch
	pub(super) git_rx: Option<
		mpsc::Receiver<GitResult<GitHistory>>,
	>,
	/// Receiver for git watcher live updates
	pub(super) git_watch_rx: Option<
		mpsc::Receiver<GitResult<GitHistory>>,
	>,
	/// Handle to stop git watcher thread
	pub(super) git_watch_handle:
		Option<GitWatchHandle>,
}

impl App {
	pub fn new(file_cache: FileCache) -> Self {
		let root = Self::resolve_project_root();
		let cfg = crate::service::config
			::load_config(std::path::Path::new(&root));
		let backend =
			crate::service::backend::backend_for_kind(
				cfg.claude.backend,
			);
		Self {
			backend,
			input: String::new(),
			cursor_position: CursorPosition::default(),
			should_quit: false,
			picker: Picker::new(file_cache),
			file_references: Vec::new(),
			symbol_selectors: Vec::new(),
			history: ConversationHistory::new(),
			status_message: None,
			claude_session_id: None,
			claude_rx: None,
			last_claude_response: None,
			streaming_text: String::new(),
			tool_status: None,
			scroll_offset: 0,
			memory_session_id:
				id_gen::new_session_id(),
			pending_user_input: None,
			model_name: model::load_model(&root),
			effort_level: model::load_effort(&root),
			tool_rx: None,
			tool_references: Vec::new(),
			mcp_discovery_rx: None,
			prompt_history: prompt_history_io::load(
				std::path::Path::new(&root),
			),
			project_root: root,
			paste_blocks: Vec::new(),
			esc_pressed_at: None,
			ctrl_c_pressed_at: None,
			watcher_msg: None,
			inline_blocks: None,
			output_area: Cell::new(Rect::default()),
			block_ranges: RefCell::new(Vec::new()),
			pending_block_question: None,
			history_index: None,
			saved_input: String::new(),
			enhance_rx: None,
			pre_enhance_input: None,
			pre_enhance_agents: None,
			preflight: None,
			agent_suggestions: Vec::new(),
			skills: skills::scan_skills(),
			git_rx: None,
			git_watch_rx: None,
			git_watch_handle: None,
		}
	}

	fn resolve_project_root() -> String {
		std::env::current_dir()
			.map(|d| d.to_string_lossy().into_owned())
			.unwrap_or_else(|_| ".".to_string())
	}

	/// Replace the active backend at runtime.
	pub fn set_backend(
		&mut self,
		backend: Arc<dyn CliBackend>,
	) {
		self.backend = backend;
	}

	pub fn sync_file_cache(&mut self) {
		self.picker.sync_cache();
	}

	pub fn handle_mouse(
		&mut self,
		col: u16,
		row: u16,
	) {
		if self.inline_blocks.is_some() {
			self.handle_inline_click(col, row);
		} else {
			self.handle_debug_click(col, row);
		}
	}
}

impl Default for App {
	fn default() -> Self {
		Self::new(FileCache::empty())
	}
}
