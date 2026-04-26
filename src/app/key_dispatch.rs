use std::time::Instant;

use crossterm::event::{KeyCode, KeyModifiers};

use crate::app::App;
use crate::ui::strings::tui_labels;

/// Window for double Ctrl+C (ms)
const DOUBLE_CTRL_C_MS: u128 = 1500;

impl App {
	/// Returns true if the app should quit.
	pub fn handle_key(
		&mut self,
		key: KeyCode,
		mods: KeyModifiers,
	) -> bool {
		self.status_message = None;
		if is_ctrl_c(key, mods) {
			return self.handle_ctrl_c();
		}
		self.ctrl_c_pressed_at = None;
		if key != KeyCode::Esc {
			self.esc_pressed_at = None;
		}
		self.dispatch_key(key, mods)
	}

	/// Double Ctrl+C within 1.5s to quit.
	fn handle_ctrl_c(&mut self) -> bool {
		if let Some(ts) = self.ctrl_c_pressed_at {
			if ts.elapsed().as_millis()
				<= DOUBLE_CTRL_C_MS
			{
				self.should_quit = true;
				return true;
			}
		}
		self.ctrl_c_pressed_at =
			Some(Instant::now());
		self.status_message = Some(
			tui_labels::QUIT_HINT.to_string(),
		);
		false
	}

	/// Route key to the active input mode.
	///
	/// Preflight panel captures Enter/Esc.
	/// Inline blocks with a focused code/question
	/// capture keys. Otherwise keys go to input bar.
	fn dispatch_key(
		&mut self,
		key: KeyCode,
		mods: KeyModifiers,
	) -> bool {
		if self.preflight.is_some() {
			return self.dispatch_preflight(key);
		}
		if self.has_inline_focus() {
			let quit = self.handle_inline_key(
				key, mods,
			);
			if key == KeyCode::Esc {
				self.esc_pressed_at = None;
			}
			return quit;
		}
		if self.picker.is_active() {
			let quit = self.handle_picker_key(key);
			if key == KeyCode::Esc {
				self.esc_pressed_at = None;
			}
			return quit;
		}
		self.handle_input_key(key, mods)
	}

	/// Handle keys while preflight panel is active.
	fn dispatch_preflight(
		&mut self,
		key: KeyCode,
	) -> bool {
		match key {
			KeyCode::Enter => {
				self.confirm_preflight();
			}
			KeyCode::Esc => {
				self.cancel_preflight();
			}
			_ => {}
		}
		false
	}

	/// True when a code block or question is focused
	fn has_inline_focus(&self) -> bool {
		self.inline_blocks
			.as_ref()
			.is_some_and(|ib| ib.focus().is_some())
	}
}

fn is_ctrl_c(key: KeyCode, mods: KeyModifiers) -> bool {
	key == KeyCode::Char('c')
		&& mods.contains(KeyModifiers::CONTROL)
}
