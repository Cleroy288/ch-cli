use crossterm::event::KeyCode;

use crate::app::App;
use crate::domain::tool_ref::{ToolItem, ToolKind};

impl App {
	/// Handle keys in McpToolBrowse mode
	pub(crate) fn handle_mcp_browse_key(
		&mut self,
		key: KeyCode,
	) -> bool {
		let count = self
			.picker
			.filtered_mcp_items()
			.len();
		match key {
			KeyCode::Up => self.picker.move_up(),
			KeyCode::Down => {
				self.picker.move_down(count);
			}
			KeyCode::Char(chr) => {
				self.picker.push_query(chr);
			}
			KeyCode::Backspace => {
				self.mcp_browse_backspace();
			}
			KeyCode::Enter => self.select_mcp_tool(),
			KeyCode::Esc => {
				self.picker.back_to_tools_from_mcp();
			}
			_ => {}
		}
		false
	}

	/// Backspace: pop query or go back to tools
	fn mcp_browse_backspace(&mut self) {
		if self.picker.query().is_empty() {
			self.picker.back_to_tools_from_mcp();
		} else {
			self.picker.pop_query();
		}
	}

	/// Select an MCP tool and insert reference
	fn select_mcp_tool(&mut self) {
		let Some(entry) =
			self.picker.selected_mcp_tool()
		else {
			return;
		};
		let item = ToolItem {
			key: entry.name.clone(),
			display: entry.name.clone(),
			description: entry.description.clone(),
		};
		self.insert_tool_reference(
			ToolKind::McpTool, &item,
		);
	}
}
