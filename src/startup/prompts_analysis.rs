//! User prompts with codebase analysis information.

use std::io::{self, Write};
use std::time::Duration;

use crossterm::{
	cursor,
	event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
	execute,
	style::{Color, Print, ResetColor, SetForegroundColor},
	terminal::{self, ClearType},
};

use crate::indexer::CodebaseAnalysis;

use super::StartupAction;

/// Prompt for indexing with optional language analysis info
pub fn prompt_for_indexing_with_analysis(
	show_partial_note: bool,
	analysis: &CodebaseAnalysis,
) -> io::Result<StartupAction> {
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

	// Show partial support note if applicable
	if show_partial_note {
		if let Some(primary) = analysis.primary_language {
			execute!(
				stdout,
				SetForegroundColor(Color::DarkGrey),
				Print(format!(
					"  Note: Primary language ({}) is not yet supported.\n",
					primary.display_name()
				)),
				ResetColor
			)?;
		}

		let supported_count = analysis.supported_file_count();
		if supported_count > 0 {
			execute!(
				stdout,
				SetForegroundColor(Color::DarkGrey),
				Print(format!("  Will index {} supported file(s).\n\n", supported_count)),
				ResetColor
			)?;
		}
	}

	println!("  First-time setup enables:");
	execute!(
		stdout,
		SetForegroundColor(Color::Green),
		Print("    - Fast symbol search across all files\n"),
		Print("    - Go-to-definition functionality\n"),
		Print("    - Find all references to a symbol\n"),
		Print("    - Semantic code understanding\n"),
		Print("    - Auto-generated documentation for all symbols\n\n"),
		ResetColor
	)?;

	execute!(
		stdout,
		SetForegroundColor(Color::DarkGrey),
		Print("  (Index + doc generation, ~2-5 min one-time setup)\n\n"),
		ResetColor
	)?;

	execute!(
		stdout,
		Print("  Would you like to set up your codebase now?\n\n"),
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
