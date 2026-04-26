use crate::fs::FsEntry;

use super::state::Picker;

impl Picker {
	pub fn push_query(&mut self, chr: char) {
		self.query.push(chr);
	}

	pub fn pop_query(&mut self) {
		self.query.pop();
	}

	pub fn clear_query(&mut self) {
		self.query.clear();
	}

	pub fn move_up(&mut self) {
		self.query.move_up();
	}

	pub fn move_down(&mut self, max_items: usize) {
		self.query.move_down(max_items);
	}

	/// Filtered filesystem entries for browse mode
	pub fn get_results(&self) -> Vec<&FsEntry> {
		self.scanner.get_results(
			&self.mode,
			self.query.query(),
		)
	}

	/// Currently selected filesystem entry
	pub fn get_selected_entry(
		&self,
	) -> Option<&FsEntry> {
		let results = self.get_results();
		results.get(self.selected_index()).copied()
	}
}
