//! Low-level draw helpers for doc generation progress.
//!
//! Renders the bar line, stats, hints, and status
//! messages during doc generation.

use std::io::{self, Write};
use std::time::Duration;

use crossterm::{
	cursor, execute,
	style::{
		Color, Print, ResetColor, SetForegroundColor,
	},
	terminal::{self, ClearType},
};

/// Show "waiting for daemon" message
pub(crate) fn display_daemon_wait(
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	execute!(
		stdout,
		terminal::Clear(ClearType::All),
		cursor::MoveTo(0, 0)
	)?;
	writeln!(stdout)?;
	execute!(
		stdout,
		SetForegroundColor(Color::Cyan),
		Print(
			"  Step 2/3: Starting daemon + \
			loading models...\n",
		),
		ResetColor
	)?;
	stdout.flush()
}

/// Show doc generation header
pub(crate) fn display_docgen_header(
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	execute!(
		stdout,
		terminal::Clear(ClearType::All),
		cursor::MoveTo(0, 0)
	)?;
	writeln!(stdout)?;
	execute!(
		stdout,
		SetForegroundColor(Color::Cyan),
		Print(
			"  Step 3/3: Generating \
			documentation...\n\n",
		),
		ResetColor
	)?;
	stdout.flush()
}

/// Show daemon startup failure
pub(crate) fn show_daemon_error(
	stdout: &mut io::Stdout,
) -> io::Result<()> {
	execute!(
		stdout,
		cursor::MoveTo(0, 3),
		terminal::Clear(ClearType::CurrentLine),
		SetForegroundColor(Color::Red),
		Print(
			"  Failed to start daemon. Run \
			'rustean daemon run' manually.\n",
		),
		ResetColor
	)?;
	std::thread::sleep(Duration::from_secs(2));
	Ok(())
}

/// Show doc gen start failure
pub(crate) fn show_start_error(
	stdout: &mut io::Stdout,
	err_msg: &str,
) -> io::Result<()> {
	execute!(
		stdout,
		cursor::MoveTo(0, 5),
		terminal::Clear(ClearType::CurrentLine),
		SetForegroundColor(Color::Red),
		Print(format!(
			"  Doc generation error: {}\n",
			err_msg
		)),
		ResetColor
	)?;
	std::thread::sleep(Duration::from_secs(2));
	Ok(())
}

/// Render the bar line with spinner and percentage
pub(crate) fn draw_bar_line(
	stdout: &mut io::Stdout,
	spinner: &str,
	bar_str: &str,
	progress: f64,
) -> io::Result<()> {
	execute!(
		stdout,
		cursor::MoveTo(0, 3),
		terminal::Clear(ClearType::CurrentLine),
		SetForegroundColor(Color::Green),
		Print(format!("  {} ", spinner)),
		SetForegroundColor(Color::Cyan),
		Print("["),
		SetForegroundColor(Color::Green),
		Print(bar_str),
		SetForegroundColor(Color::Cyan),
		Print("]"),
		SetForegroundColor(Color::White),
		Print(format!(
			" {:.0}%",
			progress * 100.0
		)),
		ResetColor
	)
}
