use crate::fs::FsEntry;

use super::state::Picker;

impl Picker {
	/// Get filtered results based on current mode/query
	pub fn get_results(&self) -> Vec<&FsEntry> {
		self.scanner
			.get_results(&self.mode, self.query.query())
	}

	/// Get the currently selected entry (if any)
	pub fn get_selected_entry(
		&self,
	) -> Option<&FsEntry> {
		let results = self.get_results();
		results.get(self.selected_index()).copied()
	}
}
