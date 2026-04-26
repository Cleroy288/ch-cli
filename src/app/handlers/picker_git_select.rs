use crossterm::event::KeyCode;

use crate::app::App;

impl App {
	/// Handle keys in GitRepoSelect mode.
	pub(crate) fn handle_git_select_key(
		&mut self,
		key: KeyCode,
	) -> bool {
		match key {
			KeyCode::Esc => {
				self.picker.clear_git_history();
				self.esc_pressed_at = None;
			}
			KeyCode::Enter => {
				self.confirm_git_repo();
			}
			KeyCode::Up => self.picker.move_up(),
			KeyCode::Down => {
				let count = self.picker
					.filtered_git_repos()
					.len();
				self.picker.move_down(count);
			}
			KeyCode::Backspace => {
				self.git_select_backspace();
			}
			KeyCode::Char(chr) => {
				self.picker.push_query(chr);
				self.picker
					.query.set_selected_index(0);
			}
			_ => {}
		}
		false
	}

	fn confirm_git_repo(&mut self) {
		let repo = self.picker
			.selected_git_repo()
			.cloned();
		if let Some(path) = repo {
			self.spawn_git_fetch(path);
		}
	}

	fn git_select_backspace(&mut self) {
		if self.picker.query().is_empty() {
			self.picker.clear_git_history();
			self.esc_pressed_at = None;
		} else {
			self.picker.pop_query();
			self.picker
				.query.set_selected_index(0);
		}
	}
}
