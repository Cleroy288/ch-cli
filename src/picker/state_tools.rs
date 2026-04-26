use crate::domain::tool_ref::{ToolItem, ToolKind};

use super::mode::PickerMode;
use super::state::Picker;

impl Picker {
	pub fn activate_tools(
		&mut self,
		position: usize,
	) {
		self.mode = PickerMode::Tools;
		self.query.clear();
		self.trigger_position = position;
		self.symbol_browser = None;
		self.tool_results.clear();
	}

	pub fn is_tools_mode(&self) -> bool {
		matches!(self.mode, PickerMode::Tools)
	}

	/// Transition to tool loading state
	pub fn activate_tool_loading(
		&mut self,
		tool: ToolKind,
	) {
		self.mode = PickerMode::ToolLoading { tool };
		self.query.clear();
		self.tool_results.clear();
		self.tool_error = None;
	}

	/// Switch to ToolResults mode, clear query
	pub fn enter_tool_results(
		&mut self,
		tool: ToolKind,
	) {
		self.mode = PickerMode::ToolResults { tool };
		self.query.clear();
	}

	/// Store fetched results, switch to results
	pub fn set_tool_results(
		&mut self,
		tool: ToolKind,
		items: Vec<ToolItem>,
	) {
		self.tool_results = items;
		self.tool_error = None;
		self.enter_tool_results(tool);
	}

	pub fn set_tool_error(
		&mut self,
		tool: ToolKind,
		msg: String,
	) {
		self.tool_error = Some(msg);
		self.tool_results.clear();
		self.enter_tool_results(tool);
	}

	pub fn back_to_tools(&mut self) {
		self.mode = PickerMode::Tools;
		self.query.clear();
		self.tool_results.clear();
		self.tool_error = None;
		self.clear_jira_board();
		self.clear_jira_projects();
	}
}
