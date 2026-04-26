use crate::app::App;

impl App {
	///
	/// Deletes the placeholder from input, moves
	/// cursor to block start, and shifts later blocks.
	/// Returns true if a block was removed.
	pub(crate) fn remove_paste_at(
		&mut self,
		pos: usize,
	) -> bool {
		let idx = self.paste_blocks.iter().position(
			|b| pos > b.start && pos <= b.end,
		);
		let Some(idx) = idx else {
			return false;
		};
		let block = self.paste_blocks.remove(idx);
		self.remove_paste_range(
			block.start, block.end,
		);
		true
	}

	/// cursor, and shift remaining paste blocks.
	fn remove_paste_range(
		&mut self,
		start: usize,
		end: usize,
	) {
		let len = end - start;
		self.input.drain(start..end);
		self.cursor_position.set(start);
		for block in &mut self.paste_blocks {
			if block.start >= end {
				block.start -= len;
				block.end -= len;
			}
		}
		self.update_file_references();
	}
}
