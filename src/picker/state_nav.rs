use std::path::PathBuf;

use crate::indexer::symbols::Symbol;
use crate::picker::mode::PickerMode;
use crate::picker::symbol_browser::SymbolBrowser;

use super::state::Picker;

impl Picker {
	/// Enter file browse mode at trigger position
	pub fn activate(&mut self, position: usize) {
		self.mode = PickerMode::Browse {
			dir: PathBuf::from("."),
		};
		self.query.clear();
		self.trigger_position = position;
		self.symbol_browser = None;
	}

	/// Return to inactive state
	pub fn deactivate(&mut self) {
		self.mode = PickerMode::Inactive;
		self.query.clear();
		self.symbol_browser = None;
	}

	/// Enter symbol browse for a parsed file
	pub fn activate_symbols(
		&mut self,
		position: usize,
		file_path: PathBuf,
		symbols: Vec<Symbol>,
	) {
		let browser =
			SymbolBrowser::new(file_path.clone(), symbols);
		self.mode = PickerMode::Symbols {
			file_path,
			parent: None,
		};
		self.query.clear();
		self.trigger_position = position;
		self.symbol_browser = Some(browser);
	}

	/// Navigate into a subdirectory
	pub fn browse_into(&mut self, dir: PathBuf) {
		self.mode = PickerMode::Browse { dir };
		self.query.clear();
	}

	pub fn is_symbol_mode(&self) -> bool {
		matches!(self.mode, PickerMode::Symbols { .. })
	}
}
