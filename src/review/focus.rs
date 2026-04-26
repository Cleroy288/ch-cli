/// Focus management for InlineBlocks.
///
/// Syncing the current editor before switching
/// prevents stale state in the TextArea.

use super::state::{InlineBlocks, InlineFocus};

impl InlineBlocks {
	/// Syncs current editor before switching
	pub fn set_focus(&mut self, f: InlineFocus) {
		self.save_current_edits();
		self.focus = Some(f);
	}

	pub fn clear_focus(&mut self) {
		self.save_current_edits();
		self.focus = None;
	}

	/// Persist editor text to `edited` field.
	/// Questions are saved inline — no sync needed.
	fn save_current_edits(&mut self) {
		let idx = match self.focus {
			Some(InlineFocus::CodeBlock(i)) => i,
			_ => return,
		};
		if let Some(b) =
			self.blocks.get_mut(idx.val())
		{
			b.sync_edited_from_editor();
		}
	}
}
