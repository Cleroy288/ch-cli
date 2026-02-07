//! Progress display for documentation generation.
//!
//! Polls daemon DocGenStatus and displays a progress bar
//! until complete.

use std::io::{self, Write};
use std::time::{Duration, Instant};

use crossterm::{
	cursor,
	event::{self, Event, KeyCode, KeyEventKind},
	execute,
	style::{Color, Print, ResetColor, SetForegroundColor},
	terminal::{self, ClearType},
};

use crate::retrieval::daemon::{DaemonClient, ensure_daemon_ready};

/// Poll interval for checking doc gen status
const POLL_INTERVAL_MS: u64 = 2000;

/// Display progress bar while generating documentation
pub fn generate_docs_with_progress() -> io::Result<()> {
	// stdout handle for terminal output
	let mut stdout = io::stdout();
	// daemon client for IPC
	let client = DaemonClient::new();
	// current project path
	let project_path = std::env::current_dir()
		.unwrap_or_default()
		.to_string_lossy()
		.to_string();

	// Step 2/3: Ensure daemon is ready
	// (auto-starts if not running)
	display_daemon_wait(&mut stdout)?;

	if !ensure_daemon_ready() {
		execute!(
			stdout,
			cursor::MoveTo(0, 3),
			terminal::Clear(ClearType::CurrentLine),
			SetForegroundColor(Color::Red),
			Print(
				"  Failed to start daemon. Run \
				'ch-cli daemon run' manually.\n"
			),
			ResetColor
		)?;
		std::thread::sleep(Duration::from_secs(2));
		return Ok(());
	}

	// Step 3/3: Start doc generation and show progress
	display_docgen_header(&mut stdout)?;

	// send StartDocGen request
	let start_result =
		client.start_doc_gen(project_path.clone(), false);
	if let Err(e) = start_result {
		execute!(
			stdout,
			cursor::MoveTo(0, 5),
			terminal::Clear(ClearType::CurrentLine),
			SetForegroundColor(Color::Red),
			Print(format!("  Doc generation error: {}\n", e)),
			ResetColor
		)?;
		std::thread::sleep(Duration::from_secs(2));
		return Ok(());
	}

	// poll progress with progress bar
	poll_docgen_progress(
		&mut stdout,
		&client,
		&project_path,
	)?;

	// done message
	display_docgen_complete(&mut stdout)?;

	Ok(())
}

/// Show "waiting for daemon" message
fn display_daemon_wait(stdout: &mut io::Stdout) -> io::Result<()> {
	execute!(
		stdout,
		terminal::Clear(ClearType::All),
		cursor::MoveTo(0, 0)
	)?;
	println!();
	execute!(
		stdout,
		SetForegroundColor(Color::Cyan),
		Print(
			"  Step 2/3: Starting daemon + \
			loading models...\n"
		),
		ResetColor
	)?;
	stdout.flush()
}

/// Show doc generation header
fn display_docgen_header(stdout: &mut io::Stdout) -> io::Result<()> {
	execute!(
		stdout,
		terminal::Clear(ClearType::All),
		cursor::MoveTo(0, 0)
	)?;
	println!();
	execute!(
		stdout,
		SetForegroundColor(Color::Cyan),
		Print("  Step 3/3: Generating documentation...\n\n"),
		ResetColor
	)?;
	stdout.flush()
}

/// Poll DocGenStatus and display progress bar until complete or cancelled
fn poll_docgen_progress(
	stdout: &mut io::Stdout,
	client: &DaemonClient,
	project_path: &str,
) -> io::Result<()> {
	let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
	let mut spinner_idx = 0; // current spinner frame
	let bar_width = 40; // progress bar width in chars
	let start_time = Instant::now(); // start time for elapsed display

	// enable raw mode for Ctrl+C detection
	terminal::enable_raw_mode()?;

	loop {
		// check for Ctrl+C
		if event::poll(Duration::from_millis(POLL_INTERVAL_MS))? {
			if let Event::Key(key) = event::read()? {
				if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('c')
					&& key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL)
				{
					terminal::disable_raw_mode()?;
					execute!(
						stdout,
						cursor::MoveTo(0, 7),
						Print("\n  Cancelled. Docs will continue in background.\n")
					)?;
					return Ok(());
				}
			}
		}

		// poll daemon for status
		let status = match client.doc_gen_status(project_path.to_string()) {
			Ok(s) => s,
			Err(_) => {
				spinner_idx += 1;
				continue;
			}
		};

		let total = status.total.max(1); // avoid division by zero
		let completed = status.completed;
		let progress = (completed as f64 / total as f64).min(1.0);
		let filled = (progress * bar_width as f64) as usize;
		let empty = bar_width - filled;

		let bar: String = format!("{}{}", "█".repeat(filled), "░".repeat(empty));
		let spinner = spinner_frames[spinner_idx % spinner_frames.len()];
		spinner_idx += 1;

		// draw progress bar
		execute!(
			stdout,
			cursor::MoveTo(0, 3),
			terminal::Clear(ClearType::CurrentLine),
			SetForegroundColor(Color::Green),
			Print(format!("  {} ", spinner)),
			SetForegroundColor(Color::Cyan),
			Print("["),
			SetForegroundColor(Color::Green),
			Print(&bar),
			SetForegroundColor(Color::Cyan),
			Print("]"),
			SetForegroundColor(Color::White),
			Print(format!(" {:.0}%", progress * 100.0)),
			ResetColor
		)?;

		// draw counts
		execute!(
			stdout,
			cursor::MoveTo(0, 5),
			terminal::Clear(ClearType::CurrentLine),
			SetForegroundColor(Color::DarkGrey),
			Print(format!("  Docs: {}/{}", completed, total)),
			ResetColor
		)?;

		// elapsed time
		let elapsed = start_time.elapsed().as_secs();
		execute!(
			stdout,
			cursor::MoveTo(0, 6),
			terminal::Clear(ClearType::CurrentLine),
			SetForegroundColor(Color::DarkGrey),
			Print(format!("  Elapsed: {}s  (Ctrl+C to skip)", elapsed)),
			ResetColor
		)?;

		stdout.flush()?;

		// check if done
		if status.is_ready || (!status.in_progress && completed > 0) {
			break;
		}
	}

	terminal::disable_raw_mode()?;
	Ok(())
}

/// Display completion message
fn display_docgen_complete(stdout: &mut io::Stdout) -> io::Result<()> {
	execute!(
		stdout,
		cursor::MoveTo(0, 0),
		terminal::Clear(ClearType::All)
	)?;
	println!();
	execute!(
		stdout,
		SetForegroundColor(Color::Green),
		Print("  Setup complete!\n\n"),
		ResetColor,
		SetForegroundColor(Color::White),
		Print("  Run 'ch-cli info <symbol>' to lookup any symbol.\n"),
		Print("  Run 'ch-cli docs show <symbol>' to view documentation.\n\n"),
		ResetColor
	)?;
	stdout.flush()?;
	std::thread::sleep(Duration::from_secs(3));
	Ok(())
}
