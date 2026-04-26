use std::io;
use std::time::Duration;

use crossterm::event::{
	self, Event, KeyCode, KeyEvent, KeyEventKind,
};

///
/// Blocks until a valid key is pressed, polling at
/// 100ms intervals.
pub fn read_ynq_key() -> io::Result<KeyAction> {
	loop {
		if !event::poll(
			Duration::from_millis(100),
		)? {
			continue;
		}
		if let Some(action) = try_read_key()? {
			return Ok(action);
		}
	}
}

/// Try to read a single key event
fn try_read_key() -> io::Result<Option<KeyAction>> {
	let Event::Key(KeyEvent {
		code,
		kind: KeyEventKind::Press,
		..
	}) = event::read()?
	else {
		return Ok(None);
	};
	match code {
		KeyCode::Char('y')
		| KeyCode::Char('Y')
		| KeyCode::Enter => Ok(Some(KeyAction::Yes)),
		KeyCode::Char('n')
		| KeyCode::Char('N') => {
			Ok(Some(KeyAction::No))
		}
		KeyCode::Char('q')
		| KeyCode::Char('Q')
		| KeyCode::Esc => Ok(Some(KeyAction::Quit)),
		_ => Ok(None),
	}
}

/// Simple Y/N/Q key action
pub enum KeyAction {
	/// User pressed Y or Enter
	Yes,
	/// User pressed N
	No,
	/// User pressed Q or Esc
	Quit,
}
