//! Progress display for full indexing.

use std::io::{self, Write};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

use crossterm::{
	cursor,
	execute,
	style::{Color, Print, ResetColor, SetForegroundColor},
	terminal::{self, ClearType},
};

use crate::indexer::{IndexManager, IndexResult};

/// Display a progress bar during indexing (like Augment/Auggie style)
pub fn index_with_progress() -> io::Result<Option<IndexResult>> {
	let mut stdout = io::stdout();

	// Clear screen
	execute!(stdout, terminal::Clear(ClearType::All), cursor::MoveTo(0, 0))?;

	println!();
	execute!(
		stdout,
		SetForegroundColor(Color::Cyan),
		Print("  Indexing codebase...\n\n"),
		ResetColor
	)?;

	// Progress tracking
	// Number of processed files
	let files_processed = Arc::new(AtomicUsize::new(0));
	// total number of files
	let total_files = Arc::new(AtomicUsize::new(0));
	// Current file name being processed
	let current_file = Arc::new(
		std::sync::Mutex::new(String::new())
	);
	// Flag indicating indexing is done
	let indexing_done = Arc::new(AtomicBool::new(false));

	let files_processed_clone = files_processed.clone();
	let total_files_clone = total_files.clone();
	let current_file_clone = current_file.clone();
	let indexing_done_clone = indexing_done.clone();

	// Start indexing in a separate thread
	let handle = std::thread::spawn(move || {
		let manager = IndexManager::new()
			.with_persistence()
			.with_semantic_analysis()
			.with_reference_extraction()
			.on_progress(move |current, total, path| {
				files_processed_clone.store(current, Ordering::SeqCst);
				total_files_clone.store(total, Ordering::SeqCst);
				if let Ok(mut file) = current_file_clone.lock() {
					*file = path
						.file_name()
						.and_then(|n| n.to_str())
						.unwrap_or("")
						.to_string();
				}
			});

		let result = manager.index_project(".");
		indexing_done_clone.store(true, Ordering::SeqCst);
		result
	});

	// Animation frames for spinner
	let spinner_frames = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
	let mut spinner_idx = 0; // current spinner frame index
	let start_time = Instant::now(); // start time for elapsed calculation

	// Progress bar width
	let bar_width = 40;

	// Display loop
	while !indexing_done.load(Ordering::SeqCst) {
		let processed = files_processed.load(Ordering::SeqCst);
		let total = total_files.load(Ordering::SeqCst).max(1);
		let current = current_file
			.lock()
			.map(|f| f.clone())
			.unwrap_or_default();

		// Calculate progress
		let progress = (processed as f64 / total as f64).min(1.0);
		let filled = (progress * bar_width as f64) as usize;
		let empty = bar_width - filled;

		// Build progress bar
		let bar: String = format!(
			"{}{}",
			"█".repeat(filled),
			"░".repeat(empty),
		);

		// Spinner
		let spinner = spinner_frames[spinner_idx % spinner_frames.len()];
		spinner_idx += 1;

		// Move cursor and clear line
		execute!(
			stdout,
			cursor::MoveTo(0, 3),
			terminal::Clear(ClearType::CurrentLine)
		)?;

		// Display progress bar
		execute!(
			stdout,
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

		// Display file count
		execute!(
			stdout,
			cursor::MoveTo(0, 5),
			terminal::Clear(ClearType::CurrentLine),
			SetForegroundColor(Color::DarkGrey),
			Print(format!("  Files: {}/{}", processed, total)),
			ResetColor
		)?;

		// Display current file (truncated if too long)
		let display_file = if current.len() > 50 {
			format!("...{}", &current[current.len() - 47..])
		} else {
			current
		};

		execute!(
			stdout,
			cursor::MoveTo(0, 6),
			terminal::Clear(ClearType::CurrentLine),
			SetForegroundColor(Color::DarkGrey),
			Print(format!("  Current: {}", display_file)),
			ResetColor
		)?;

		// Elapsed time
		let elapsed = start_time.elapsed().as_secs();
		execute!(
			stdout,
			cursor::MoveTo(0, 7),
			terminal::Clear(ClearType::CurrentLine),
			SetForegroundColor(Color::DarkGrey),
			Print(format!("  Elapsed: {}s", elapsed)),
			ResetColor
		)?;

		stdout.flush()?;

		// Small delay to avoid excessive CPU usage
		std::thread::sleep(Duration::from_millis(80));
	}

	// Wait for indexing thread to complete
	let result = handle
		.join()
		.map_err(|_| {
			io::Error::new(
				io::ErrorKind::Other,
				"Indexing thread panicked",
			)
		})?;

	// Clear the progress display
	execute!(
		stdout,
		cursor::MoveTo(0, 0),
		terminal::Clear(ClearType::All)
	)?;

	match result {
		Ok(index_result) => Ok(Some(index_result)),
		Err(_) => Ok(None),
	}
}
