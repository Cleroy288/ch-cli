use std::io;
use std::time::Duration;

use crossterm::event::{
	self, Event, KeyCode, KeyEventKind,
};

/// User's choice from the menu
pub enum Choice {
	/// CLI mode
	Cli,
	/// API mode
	Api,
	/// Quit
	Quit,
}

pub fn read_choice_key() -> io::Result<Choice> {
	loop {
		if !event::poll(
			Duration::from_millis(100),
		)? {
			continue;
		}
		if let Some(choice) = try_parse_key()? {
			return Ok(choice);
		}
	}
}

/// Try to parse a key event into a Choice
fn try_parse_key(
) -> io::Result<Option<Choice>> {
	let Event::Key(evt) = event::read()? else {
		return Ok(None);
	};
	if evt.kind != KeyEventKind::Press {
		return Ok(None);
	}
	match evt.code {
		KeyCode::Char('1') | KeyCode::Enter => {
			Ok(Some(Choice::Cli))
		}
		KeyCode::Char('2') => {
			Ok(Some(Choice::Api))
		}
		KeyCode::Char('q')
		| KeyCode::Char('Q')
		| KeyCode::Esc => {
			Ok(Some(Choice::Quit))
		}
		_ => Ok(None),
	}
}
