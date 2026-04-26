use crate::domain::tool_ref::ToolReference;
use crate::domain::{FileReference, SymbolSelector};
use crate::picker::Picker;

use super::App;

impl App {
	/// Picker state (mode, query, selection)
	pub fn picker(&self) -> &Picker {
		&self.picker
	}

	/// Resolved @file references in the input
	pub fn file_references(
		&self,
	) -> &[FileReference] {
		&self.file_references
	}

	/// Resolved @file(symbol) selectors in the input
	pub fn symbol_selectors(
		&self,
	) -> &[SymbolSelector] {
		&self.symbol_selectors
	}

	/// Resolved #tool references in the input
	pub fn tool_references(
		&self,
	) -> &[ToolReference] {
		&self.tool_references
	}

	/// True while a background tool fetch is running
	pub fn is_tool_loading(&self) -> bool {
		self.tool_rx.is_some()
	}
}
