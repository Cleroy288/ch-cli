//! Tools picker key handler.
//!
//! Handles keyboard events when the user is browsing
//! the tools list activated by '#'.

use std::path::Path;

use crossterm::event::KeyCode;

use crate::app::App;
use crate::picker::doc_browser::{
	DocBrowser, DocBrowserEntry,
};

/// Number of tools currently available
const TOOLS_COUNT: usize = 1;

impl App {
	/// Handle keyboard events in Tools mode
	pub(crate) fn handle_tools_key(
		&mut self,
		key: KeyCode,
	) -> bool {
		match key {
			KeyCode::Up => self.picker.move_up(),
			KeyCode::Down => {
				self.picker.move_down(TOOLS_COUNT);
			}
			KeyCode::Enter => self.handle_tool_enter(),
			KeyCode::Esc => {
				self.remove_trigger_char();
				self.picker.deactivate();
			}
			_ => {}
		}
		false
	}

	/// Handle Enter on a tool item
	fn handle_tool_enter(&mut self) {
		let idx = self.picker.selected_index();
		if idx == 0 {
			self.open_doc_browser();
		}
	}

	/// Load DocStore and switch to DocBrowser mode
	fn open_doc_browser(&mut self) {
		let project = &self.project_path;
		let entries = load_doc_entries(project);
		let browser = DocBrowser::new(entries);
		self.picker.activate_doc_browser(browser);
	}
}

/// Load doc entries from DocStore on disk.
///
/// Returns empty vec if DocStore is missing or
/// cannot be loaded.
fn load_doc_entries(
	project: &str,
) -> Vec<DocBrowserEntry> {
	use crate::app::handlers::doc_preview_fetch
		::strip_project_prefix;
	use crate::retrieval::docgen::DocStore;

	let path = Path::new(project);
	let Ok(store) = DocStore::load(path) else {
		return Vec::new();
	};
	store
		.all_entries()
		.filter(|entry| entry.is_ready())
		.map(|entry| {
			let rel = strip_project_prefix(
				&entry.file_path.display().to_string(),
				project,
			);
			DocBrowserEntry {
				name: entry.name.clone(),
				kind: entry.kind.to_string(),
				rel_path: rel,
				line: entry.line,
				llm_doc: entry.llm_doc.clone(),
			}
		})
		.collect()
}
