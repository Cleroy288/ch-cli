use super::mcp_display::{
	McpDisplayItem, build_unified_list, matches_query,
};
use super::mode::PickerMode;
use super::state::Picker;

impl Picker {
	pub fn activate_mcp_browse(&mut self) {
		self.mode = PickerMode::McpToolBrowse;
		self.query.clear();
	}

	pub fn selected_mcp_tool(
		&self,
	) -> Option<McpDisplayItem> {
		let items = self.filtered_mcp_items();
		let idx = self.query.selected_index();
		items.into_iter().nth(idx)
	}

	pub fn filtered_mcp_items(
		&self,
	) -> Vec<McpDisplayItem> {
		let all = build_unified_list(
			&self.discovered_mcp_tools,
		);
		let query = self.query.query();
		if query.is_empty() {
			return all;
		}
		let lower = query.to_lowercase();
		all.into_iter()
			.filter(|item| matches_query(item, &lower))
			.collect()
	}

	/// Store discovered external MCP tools
	pub fn set_discovered_tools(
		&mut self,
		items: Vec<McpDisplayItem>,
	) {
		self.discovered_mcp_tools = items;
	}

	/// Go back from MCP browse to tools menu
	pub fn back_to_tools_from_mcp(&mut self) {
		self.mode = PickerMode::Tools;
		self.query.clear();
	}
}
