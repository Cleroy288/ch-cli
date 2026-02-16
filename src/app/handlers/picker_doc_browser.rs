//! Doc browser picker key handler.
//!
//! Handles keyboard events when browsing generated
//! documentation entries in the doc browser picker.

use crossterm::event::KeyCode;

use crate::app::App;
use crate::retrieval::daemon::protocol::DocEntryResponse;

impl App {
	/// Handle keyboard events in DocBrowser mode
	pub(crate) fn handle_doc_browser_key(
		&mut self,
		key: KeyCode,
	) -> bool {
		match key {
			KeyCode::Up => self.picker.move_up(),
			KeyCode::Down => {
				let count = self.doc_items_count();
				self.picker.move_down(count);
			}
			KeyCode::Enter => {
				self.handle_doc_entry_select();
			}
			KeyCode::Esc => {
				self.remove_trigger_char();
				self.picker.deactivate();
			}
			KeyCode::Backspace => {
				self.picker.pop_query();
			}
			KeyCode::Char(chr) => {
				self.picker.push_query(chr);
			}
			_ => {}
		}
		false
	}

	/// Count of current visible doc items
	fn doc_items_count(&self) -> usize {
		let query = self.picker.query();
		self.picker
			.doc_browser()
			.map(|br| br.current_items(query).len())
			.unwrap_or(0)
	}

	/// Select a doc entry and show its preview
	fn handle_doc_entry_select(&mut self) {
		let query = self.picker.query().to_string();
		let idx = self.picker.selected_index();
		let Some(browser) = self.picker.doc_browser()
		else {
			return;
		};
		let Some(entry) = browser.entry_at(&query, idx)
		else {
			return;
		};
		self.doc_preview =
			Some(build_doc_response(entry));
		self.doc_fetch_no_result = false;
		self.remove_trigger_char();
		self.picker.deactivate();
	}
}

/// Build DocEntryResponse from a doc browser entry
fn build_doc_response(
	entry: &crate::picker::doc_browser::DocBrowserEntry,
) -> DocEntryResponse {
	DocEntryResponse {
		name: entry.name.clone(),
		kind: entry.kind.clone(),
		file_path: entry.rel_path.clone(),
		line: entry.line,
		user_comment: None,
		llm_doc: entry.llm_doc.clone(),
		signature: None,
		depends_on: Vec::new(),
		depended_by: Vec::new(),
		external_deps: Vec::new(),
		status: "Ready".to_string(),
	}
}
