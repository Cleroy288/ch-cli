use crate::app::App;
use crate::domain::review::BlockIdx;
use crate::message::MessageSegment;
use crate::service::review_write;

impl App {
	pub(crate) fn submit_block_question(
		&mut self,
		idx: BlockIdx,
	) {
		let prompt = match &mut self.inline_blocks
		{
			Some(ib) => ib.submit_question(idx),
			None => return,
		};
		let Some(prompt) = prompt else { return };
		self.pending_block_question = Some(idx);
		let segments =
			vec![MessageSegment::Text(prompt)];
		crate::app::claude_request
			::spawn_claude_request(self, &segments);
	}

	pub(crate) fn execute_impl_step(&mut self) {
		let pair = match &mut self.inline_blocks {
			Some(ib) => ib.confirm_step(),
			None => return,
		};
		let Some((path, content)) = pair else {
			return;
		};
		let msg = match review_write::write_single_file(
			&path, &content,
		) {
			Ok(()) => format!("Wrote {}", path),
			Err(e) => {
				format!("Write failed: {}", e)
			}
		};
		self.set_status_message(Some(msg));
	}
}
