use crossterm::event::KeyCode;

use crate::app::App;

impl App {
	/// Handle keys while git log is loading.
	pub(crate) fn handle_git_loading_key(
		&mut self,
		key: KeyCode,
	) -> bool {
		if key == KeyCode::Esc {
			self.close_git_history();
		}
		false
	}

	/// Handle keys in GitHistory list mode.
	pub(crate) fn handle_git_history_key(
		&mut self,
		key: KeyCode,
	) -> bool {
		match key {
			KeyCode::Up | KeyCode::Char('k') => {
				self.picker.select_git_up();
			}
			KeyCode::Down | KeyCode::Char('j') => {
				self.picker.select_git_down();
			}
			KeyCode::Enter => {
				self.picker.enter_git_detail();
			}
			KeyCode::Char('r') => {
				self.refresh_git_history();
			}
			KeyCode::Esc | KeyCode::Backspace => {
				self.close_git_history();
			}
			_ => {}
		}
		false
	}

	/// Handle keys in GitDetail mode.
	pub(crate) fn handle_git_detail_key(
		&mut self,
		key: KeyCode,
	) -> bool {
		match key {
			KeyCode::Up | KeyCode::Char('k') => {
				self.picker.scroll_git_detail_up();
			}
			KeyCode::Down | KeyCode::Char('j') => {
				self.picker
					.scroll_git_detail_down();
			}
			KeyCode::Enter => {
				self.insert_git_reference();
			}
			KeyCode::Esc | KeyCode::Backspace => {
				self.picker.back_to_git_list();
			}
			_ => {}
		}
		false
	}
}
