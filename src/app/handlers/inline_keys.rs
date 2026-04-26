use crossterm::event::{
	Event, KeyCode, KeyEvent, KeyModifiers,
};

use crate::app::App;
use crate::domain::review::BlockIdx;
use crate::review::InlineFocus;

impl App {
	/// Handle key when a block has focus.
	///
	/// Esc clears focus (returns to input bar).
	/// Impl-step mode intercepts Enter/s/Esc.
	pub(crate) fn handle_inline_key(
		&mut self,
		key: KeyCode,
		mods: KeyModifiers,
	) -> bool {
		let Some(ref ib) = self.inline_blocks
		else {
			return false;
		};
		if ib.impl_step().is_some() {
			return self.handle_impl_key(key);
		}
		match ib.focus() {
			Some(InlineFocus::CodeBlock(idx)) => {
				self.handle_code_focus(
					idx, key, mods,
				)
			}
			Some(InlineFocus::Question(idx)) => {
				self.handle_question_focus(idx, key)
			}
			None => false,
		}
	}

	fn handle_code_focus(
		&mut self,
		idx: BlockIdx,
		key: KeyCode,
		mods: KeyModifiers,
	) -> bool {
		if key == KeyCode::Esc {
			if let Some(ib) =
				&mut self.inline_blocks
			{
				ib.clear_focus();
			}
			return false;
		}
		let event = Event::Key(
			KeyEvent::new(key, mods),
		);
		if let Some(ib) = &mut self.inline_blocks {
			if let Some(b) = ib.block_mut(idx) {
				b.editor.input(event);
				b.sync_edited_from_editor();
			}
		}
		false
	}

	fn handle_question_focus(
		&mut self,
		idx: BlockIdx,
		key: KeyCode,
	) -> bool {
		match key {
			KeyCode::Esc => {
				if let Some(ib) =
					&mut self.inline_blocks
				{
					ib.clear_focus();
				}
			}
			KeyCode::Enter => {
				self.submit_block_question(idx);
			}
			KeyCode::Backspace => {
				self.edit_question(idx, |q| {
					q.pop();
				});
			}
			KeyCode::Char(c) => {
				self.edit_question(idx, |q| {
					q.push(c);
				});
			}
			_ => {}
		}
		false
	}

	fn edit_question(
		&mut self,
		idx: BlockIdx,
		f: impl FnOnce(&mut String),
	) {
		let ib = match &mut self.inline_blocks {
			Some(ib) => ib,
			None => return,
		};
		if let Some(b) = ib.block_mut(idx) {
			f(&mut b.question);
		}
	}
}
