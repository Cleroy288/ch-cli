use crate::app::App;

impl App {
	pub(crate) fn handle_backspace(&mut self) {
		let cursor_pos = self.cursor_position.get();
		if cursor_pos == 0 {
			return;
		}
		if self.remove_paste_at(cursor_pos) {
			return;
		}
		self.cursor_position.move_left(&self.input);
		let new_pos = self.cursor_position.get();
		self.input.remove(new_pos);
		self.update_file_references();
	}

	pub(crate) fn handle_delete(&mut self) {
		let cursor_pos = self.cursor_position.get();
		if cursor_pos >= self.input.len() {
			return;
		}
		let next = cursor_pos + 1;
		if self.remove_paste_at(next) {
			return;
		}
		self.input.remove(cursor_pos);
		self.update_file_references();
	}
}
