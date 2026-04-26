/// Step-by-step implement flow: confirm or skip
/// each writable block in sequence.

use crate::domain::review::BlockIdx;

use super::state::{BlockMode, InlineBlocks};

impl InlineBlocks {
	pub fn start_implement(&mut self) {
		if matches!(self.mode, BlockMode::Frozen)
			|| self.blocks.is_empty()
		{
			return;
		}
		self.clear_focus();
		self.impl_step = Some(BlockIdx(0));
		self.advance_to_writable(0);
	}

	/// Confirm current step, returning (path, code)
	pub fn confirm_step(
		&mut self,
	) -> Option<(String, String)> {
		let idx = self.impl_step?;
		let block =
			self.blocks.get(idx.val())?;
		let pair = block
			.file_path
			.clone()
			.map(|p| (p, block.edited.clone()));
		self.advance_to_writable(idx.val() + 1);
		pair
	}

	pub fn skip_step(&mut self) {
		if let Some(idx) = self.impl_step {
			self.advance_to_writable(idx.val() + 1);
		}
	}

	pub fn cancel_implement(&mut self) {
		self.impl_step = None;
	}

	/// Find next writable block; freeze if none
	fn advance_to_writable(
		&mut self,
		start: usize,
	) {
		let tail =
			self.blocks.get(start..).unwrap_or(&[]);
		self.impl_step = tail
			.iter()
			.position(|b| b.file_path.is_some())
			.map(|i| BlockIdx(i + start));
		if self.impl_step.is_none() {
			self.mode = BlockMode::Frozen;
		}
	}

	pub fn writable_count(&self) -> usize {
		self.blocks
			.iter()
			.filter(|b| b.file_path.is_some())
			.count()
	}

	/// 1-based step number for display
	pub fn impl_step_display(&self) -> usize {
		let Some(idx) = self.impl_step else {
			return 0;
		};
		self.blocks[..=idx.val()]
			.iter()
			.filter(|b| b.file_path.is_some())
			.count()
	}
}
