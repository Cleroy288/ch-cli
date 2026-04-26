use crate::domain::jira::JiraBoardData;
use crate::domain::tool_ref::ToolKind;

use super::state::Picker;
use super::state_jira_convert::{
	derive_tool_items, extract_assignees,
	extract_project_keys,
};

impl Picker {
	/// Store board data, derive items, route to
	/// project select or results.
	pub fn set_jira_board(
		&mut self,
		board: JiraBoardData,
	) {
		self.tool_results =
			derive_tool_items(&board);
		self.jira_assignees =
			extract_assignees(&board.tickets);
		let keys =
			extract_project_keys(&board.tickets);
		self.jira_board = Some(board);
		self.tool_error = None;
		self.route_after_board(keys);
	}

	pub fn jira_board(
		&self,
	) -> Option<&JiraBoardData> {
		self.jira_board.as_ref()
	}

	pub fn clear_jira_board(&mut self) {
		self.jira_board = None;
		self.jira_assignees.clear();
		self.jira_assignee_filter = None;
	}

	/// Multi-project => select, single => results
	fn route_after_board(
		&mut self,
		keys: Vec<String>,
	) {
		if keys.len() > 1 {
			self.enter_space_select(keys);
		} else {
			self.jira_selected_project =
				keys.into_iter().next();
			self.enter_tool_results(ToolKind::Jira);
		}
	}
}
