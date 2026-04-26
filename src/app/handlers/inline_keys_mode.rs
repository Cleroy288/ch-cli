use crossterm::event::KeyCode;

use crate::app::App;

impl App {
	pub(super) fn handle_impl_key(
		&mut self,
		key: KeyCode,
	) -> bool {
		if self.route_scroll_key(key) {
			return false;
		}
		match key {
			KeyCode::Enter => {
				self.execute_impl_step();
			}
			KeyCode::Char('s') => {
				if let Some(ib) =
					&mut self.inline_blocks
				{
					ib.skip_step();
				}
			}
			KeyCode::Esc => {
				if let Some(ib) =
					&mut self.inline_blocks
				{
					ib.cancel_implement();
				}
			}
			_ => {}
		}
		false
	}

	/// Returns true if key was consumed as scroll
	pub(super) fn route_scroll_key(
		&mut self,
		key: KeyCode,
	) -> bool {
		let Some(ib) = &self.inline_blocks else {
			return false;
		};
		match key {
			KeyCode::Up => ib.scroll_up(),
			KeyCode::Down => ib.scroll_down(),
			KeyCode::PageUp => ib.scroll_page_up(),
			KeyCode::PageDown => {
				ib.scroll_page_down()
			}
			_ => return false,
		}
		true
	}
}
