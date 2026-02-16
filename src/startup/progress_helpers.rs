//! Shared helpers for progress display screens.

use std::io;
use std::time::Duration;

use crossterm::{
	cursor,
	event::{
		self, Event, KeyCode, KeyEventKind,
		KeyModifiers,
	},
	execute,
	style::Print,
	terminal,
};

/// Result of checking user input during progress
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProgressKeyAction {
	/// No key pressed (continue polling)
	None,
	/// User pressed Ctrl+C (cancel)
	Cancel,
	/// User pressed Z (send to background)
	Background,
}

/// Check for Ctrl+C or Z during progress polling
///
/// Returns which action the user took, if any.
pub fn check_progress_key(
	stdout: &mut io::Stdout,
) -> io::Result<ProgressKeyAction> {
	let timeout = Duration::from_millis(
		super::progress_docgen::POLL_INTERVAL_MS,
	);
	if !event::poll(timeout)? {
		return Ok(ProgressKeyAction::None);
	}
	let action = read_progress_event()?;
	if action == ProgressKeyAction::None {
		return Ok(action);
	}
	terminal::disable_raw_mode()?;
	show_background_message(stdout, action)?;
	Ok(action)
}

/// Show message when user backgrounds or cancels
fn show_background_message(
	stdout: &mut io::Stdout,
	action: ProgressKeyAction,
) -> io::Result<()> {
	let msg = match action {
		ProgressKeyAction::Cancel => {
			"\n  Cancelled. Docs continue in \
			background.\n"
		}
		ProgressKeyAction::Background => {
			"\n  Continuing in background...\n"
		}
		ProgressKeyAction::None => return Ok(()),
	};
	execute!(
		stdout,
		cursor::MoveTo(0, 7),
		Print(msg)
	)
}

/// Read a key event and classify it
fn read_progress_event(
) -> io::Result<ProgressKeyAction> {
	let Event::Key(key) = event::read()? else {
		return Ok(ProgressKeyAction::None);
	};
	if key.kind != KeyEventKind::Press {
		return Ok(ProgressKeyAction::None);
	}
	classify_key(key.code, key.modifiers)
}

/// Classify a key press into a progress action
fn classify_key(
	code: KeyCode,
	modifiers: KeyModifiers,
) -> io::Result<ProgressKeyAction> {
	let is_ctrl_c = code == KeyCode::Char('c')
		&& modifiers.contains(KeyModifiers::CONTROL);
	if is_ctrl_c {
		return Ok(ProgressKeyAction::Cancel);
	}
	let is_background =
		matches!(code, KeyCode::Char('b'));
	if is_background {
		return Ok(ProgressKeyAction::Background);
	}
	Ok(ProgressKeyAction::None)
}
