use std::path::PathBuf;

use crate::domain::git_graph::GitHistory;
use crate::domain::jira::JiraBoardData;
use crate::domain::jira_detail::JiraIssueDetail;
use crate::domain::repo_info::RepoEntry;
use crate::domain::tool_ref::ToolItem;
use crate::fs::FileCache;
use crate::picker::mcp_display::McpDisplayItem;
use crate::picker::symbol_browser::SymbolBrowser;
use crate::picker::{PickerMode, PickerQuery, PickerScanner};

/// File/folder/symbol picker state.
pub struct Picker {
	pub(crate) mode: PickerMode,
	pub(crate) trigger_position: usize,
	pub(crate) query: PickerQuery,
	pub(crate) scanner: PickerScanner,
	pub(crate) symbol_browser: Option<SymbolBrowser>,
	pub(crate) tool_results: Vec<ToolItem>,
	pub(crate) tool_error: Option<String>,
	pub(crate) repo_entries: Vec<RepoEntry>,
	pub(crate) discovered_mcp_tools: Vec<McpDisplayItem>,
	pub(crate) jira_board: Option<JiraBoardData>,
	pub(crate) jira_assignees: Vec<String>,
	pub(crate) jira_assignee_filter: Option<String>,
	pub(crate) jira_detail: Option<JiraIssueDetail>,
	pub(crate) jira_detail_scroll: usize,
	pub(crate) jira_project_keys: Vec<String>,
	pub(crate) jira_selected_project: Option<String>,
	pub(crate) git_repos: Vec<PathBuf>,
	pub(crate) git_history:
		Option<GitHistory>,
	pub(crate) git_selected: usize,
	pub(crate) git_detail_scroll: usize,
}

impl Picker {
	pub fn new(cache: FileCache) -> Self {
		Self {
			mode: PickerMode::Inactive,
			trigger_position: 0,
			query: PickerQuery::new(),
			scanner: PickerScanner::new(cache),
			symbol_browser: None,
			tool_results: Vec::new(),
			tool_error: None,
			repo_entries: Vec::new(),
			discovered_mcp_tools: Vec::new(),
			jira_board: None,
			jira_assignees: Vec::new(),
			jira_assignee_filter: None,
			jira_detail: None,
			jira_detail_scroll: 0,
			jira_project_keys: Vec::new(),
			jira_selected_project: None,
			git_repos: Vec::new(),
			git_history: None,
			git_selected: 0,
			git_detail_scroll: 0,
		}
	}

	pub fn sync_cache(&mut self) {
		self.scanner.sync_if_dirty();
	}

	pub fn is_active(&self) -> bool {
		!matches!(self.mode, PickerMode::Inactive)
	}

	pub fn mode(&self) -> &PickerMode {
		&self.mode
	}

	pub fn query(&self) -> &str {
		self.query.query()
	}

	pub fn selected_index(&self) -> usize {
		self.query.selected_index()
	}

	pub fn trigger_position(&self) -> usize {
		self.trigger_position
	}

	pub fn symbol_browser(
		&self,
	) -> Option<&SymbolBrowser> {
		self.symbol_browser.as_ref()
	}

	pub fn symbol_browser_mut(
		&mut self,
	) -> Option<&mut SymbolBrowser> {
		self.symbol_browser.as_mut()
	}
}

impl Default for Picker {
	fn default() -> Self {
		Self::new(FileCache::empty())
	}
}
