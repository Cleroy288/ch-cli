//! Picker state methods for tools and doc browser.
//!
//! Activation methods for the Tools and DocBrowser
//! picker modes, plus doc_browser accessor.

use super::doc_browser::DocBrowser;
use super::mode::PickerMode;
use super::state::Picker;

impl Picker {
	/// Activate tools picker at trigger position
	pub fn activate_tools(&mut self, position: usize) {
		self.mode = PickerMode::Tools;
		self.query.clear();
		self.trigger_position = position;
		self.symbol_browser = None;
		self.doc_browser = None;
	}

	/// Switch to doc browser mode with loaded entries
	pub fn activate_doc_browser(
		&mut self,
		browser: DocBrowser,
	) {
		self.mode = PickerMode::DocBrowser;
		self.query.clear();
		self.doc_browser = Some(browser);
	}

	/// Get the doc browser (if in DocBrowser mode)
	pub fn doc_browser(&self) -> Option<&DocBrowser> {
		self.doc_browser.as_ref()
	}

	/// Check if picker is in Tools mode
	pub fn is_tools_mode(&self) -> bool {
		matches!(self.mode, PickerMode::Tools)
	}
}
