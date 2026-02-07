//! User prompts for indexing decisions.

use std::io::{self, Write};
use std::time::Duration;

use crossterm::{
	cursor,
	event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
	execute,
	style::{Color, Print, ResetColor, SetForegroundColor},
	terminal::{self, ClearType},
};

use crate::indexer::ChangeSet;

use super::StartupAction;

/// Prompt the user to decide whether to update the index (changes detected)
pub fn prompt_for_update(changes: &ChangeSet) -> io::Result<StartupAction> {
	let mut stdout = io::stdout();

	// Clear screen and show prompt
	execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

	// Display the prompt
	println!();
	execute!(
		stdout,
		SetForegroundColor(Color::Cyan),
		Print("  ch-cli"),
		ResetColor,
		Print(" - Semantic Code Indexer\n\n")
	)?;

	execute!(
		stdout,
		SetForegroundColor(Color::Yellow),
		Print("  Changes detected in your codebase!\n\n"),
		ResetColor
	)?;

	// Show change summary
	execute!(stdout, SetForegroundColor(Color::DarkGrey))?;

	if !changes.added.is_empty() {
		execute!(
			stdout,
			SetForegroundColor(Color::Green),
			Print(format!("    + {} new file(s)\n", changes.added.len())),
		)?;
	}
	if !changes.modified.is_empty() {
		execute!(
			stdout,
			SetForegroundColor(Color::Yellow),
			Print(format!("    ~ {} modified file(s)\n", changes.modified.len())),
		)?;
	}
	if !changes.deleted.is_empty() {
		execute!(
			stdout,
			SetForegroundColor(Color::Red),
			Print(format!("    - {} deleted file(s)\n", changes.deleted.len())),
		)?;
	}

	execute!(stdout, ResetColor, Print("\n"))?;

	execute!(
		stdout,
		Print("  Would you like to update your index?\n\n"),
		SetForegroundColor(Color::White),
		Print("    ["),
		SetForegroundColor(Color::Green),
		Print("Y"),
		SetForegroundColor(Color::White),
		Print("]es  "),
		Print("["),
		SetForegroundColor(Color::Red),
		Print("N"),
		SetForegroundColor(Color::White),
		Print("]o  "),
		Print("["),
		SetForegroundColor(Color::Yellow),
		Print("Q"),
		SetForegroundColor(Color::White),
		Print("]uit\n\n"),
		ResetColor
	)?;

	execute!(
		stdout,
		SetForegroundColor(Color::DarkGrey),
		Print("  Press Y, N, or Q: "),
		ResetColor
	)?;
	stdout.flush()?;

	// Enable raw mode to capture single key press
	terminal::enable_raw_mode()?;

	let result = loop {
		if event::poll(Duration::from_millis(100))? {
			if let Event::Key(KeyEvent {
				code,
				kind: KeyEventKind::Press,
				..
			}) = event::read()?
			{
				match code {
					KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
						break StartupAction::Update(changes.clone());
					}
					KeyCode::Char('n') | KeyCode::Char('N') => {
						break StartupAction::Skip;
					}
					KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
						break StartupAction::Quit;
					}
					_ => {}
				}
			}
		}
	};

	terminal::disable_raw_mode()?;
	println!();

	Ok(result)
}

/// Prompt the user to decide whether to index the codebase
pub fn prompt_for_indexing() -> io::Result<StartupAction> {
	let mut stdout = io::stdout();

	// Clear screen and show prompt
	execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

	// Display the prompt
	println!();
	execute!(
		stdout,
		SetForegroundColor(Color::Cyan),
		Print("  ch-cli"),
		ResetColor,
		Print(" - Semantic Code Indexer\n\n")
	)?;

	execute!(
		stdout,
		SetForegroundColor(Color::Yellow),
		Print("  No code index found for this project.\n\n"),
		ResetColor
	)?;

	println!("  Indexing your codebase enables:");
	execute!(
		stdout,
		SetForegroundColor(Color::Green),
		Print("    - Fast symbol search across all files\n"),
		Print("    - Go-to-definition functionality\n"),
		Print("    - Find all references to a symbol\n"),
		Print("    - Semantic code understanding\n\n"),
		ResetColor
	)?;

	execute!(
		stdout,
		Print("  Would you like to index your codebase now?\n\n"),
		SetForegroundColor(Color::White),
		Print("    ["),
		SetForegroundColor(Color::Green),
		Print("Y"),
		SetForegroundColor(Color::White),
		Print("]es  "),
		Print("["),
		SetForegroundColor(Color::Red),
		Print("N"),
		SetForegroundColor(Color::White),
		Print("]o  "),
		Print("["),
		SetForegroundColor(Color::Yellow),
		Print("Q"),
		SetForegroundColor(Color::White),
		Print("]uit\n\n"),
		ResetColor
	)?;

	execute!(
		stdout,
		SetForegroundColor(Color::DarkGrey),
		Print("  Press Y, N, or Q: "),
		ResetColor
	)?;
	stdout.flush()?;

	// Enable raw mode to capture single key press
	terminal::enable_raw_mode()?;

	let result = loop {
		if event::poll(Duration::from_millis(100))? {
			if let Event::Key(KeyEvent {
				code,
				kind: KeyEventKind::Press,
				..
			}) = event::read()?
			{
				match code {
					KeyCode::Char('y') | KeyCode::Char('Y') | KeyCode::Enter => {
						break StartupAction::Index;
					}
					KeyCode::Char('n') | KeyCode::Char('N') => {
						break StartupAction::Skip;
					}
					KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => {
						break StartupAction::Quit;
					}
					_ => {}
				}
			}
		}
	};

	terminal::disable_raw_mode()?;
	println!();

	Ok(result)
}
