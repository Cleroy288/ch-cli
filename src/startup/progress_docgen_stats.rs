//! Stats and completion display for doc gen progress.
//!
//! Renders doc count, elapsed time, key hints, and
//! the final completion message.

use std::io::{self, Write};
use std::time::{Duration, Instant};

use crossterm::{
	cursor, execute,
	style::{
		Color, Print, ResetColor, SetForegroundColor,
	},
	terminal::{self, ClearType},
};

/// Draw doc count and elapsed time with hints
pub(crate) fn draw_docgen_stats(
	stdout: &mut io::Stdout,
	completed: usize,
	total: usize,
	start: &Instant,
) -> io::Result<()> {
	let elapsed = start.elapsed().as_secs();
	execute!(
		stdout,
		cursor::MoveTo(0, 5),
		terminal::Clear(ClearType::CurrentLine),
		SetForegroundColor(Color::DarkGrey),
		Print(format!(
			"  Docs: {}/{}",
			completed, total
		)),
		ResetColor
	)?;
	draw_hint_line(stdout, elapsed)
}

/// Draw elapsed time and key hints
fn draw_hint_line(
	stdout: &mut io::Stdout,
	elapsed: u64,
) -> io::Result<()> {
	execute!(
		stdout,
		cursor::MoveTo(0, 6),
		terminal::Clear(ClearType::CurrentLine),
		SetForegroundColor(Color::DarkGrey),
		Print(format!("  Elapsed: {}s", elapsed)),
		ResetColor,
		Print("  "),
		SetForegroundColor(Color::Yellow),
		Print("B"),
		SetForegroundColor(Color::DarkGrey),
		Print(": background  "),
		Print("Ctrl+C: skip"),
		ResetColor
	)
}

/// Display completion message after doc gen
pub(crate) fn display_docgen_complete(
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	execute!(
		stdout,
		cursor::MoveTo(0, 0),
		terminal::Clear(ClearType::All)
	)?;
	writeln!(stdout)?;
	execute!(
		stdout,
		SetForegroundColor(Color::Green),
		Print("  Setup complete!\n\n"),
		ResetColor,
		SetForegroundColor(Color::White),
		Print(
			"  Run 'rustean info <symbol>' \
			to lookup any symbol.\n",
		),
		Print(
			"  Run 'rustean docs show <symbol>' \
			to view documentation.\n\n",
		),
		ResetColor
	)?;
	stdout.flush()?;
	std::thread::sleep(Duration::from_secs(3));
	Ok(())
}
