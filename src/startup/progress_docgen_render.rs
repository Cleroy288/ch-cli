//! Poll loop for doc generation progress bar.
//!
//! Handles the main poll loop that checks daemon
//! status and user key input.

use std::io::{self, Write};
use std::time::Instant;

use crossterm::terminal;

use crate::retrieval::daemon::{
	DaemonClient, DocGenStatus,
};

use super::progress_docgen_draw::draw_bar_line;
use super::progress_docgen_stats::draw_docgen_stats;
use super::progress_helpers::{
	check_progress_key, ProgressKeyAction,
};
use super::progress_shared::{
	SPINNER, format_progress_bar,
};

/// Mutable state for the poll loop
struct PollState<'a> {
	/// spinner frame index
	spin_idx: usize,
	/// timer since poll started
	start: Instant,
	/// daemon client reference
	client: &'a DaemonClient,
	/// project path string
	project_path: &'a str,
}

/// Poll DocGenStatus and display progress bar.
///
/// Returns true if user pressed 'B' to background.
pub(crate) fn poll_docgen_progress(
	stdout: &mut io::Stdout,
	client: &DaemonClient,
	project_path: &str,
) -> io::Result<bool> {
	terminal::enable_raw_mode()?;
	let mut state = PollState {
		spin_idx: 0,
		start: Instant::now(),
		client,
		project_path,
	};

	let backgrounded =
		run_poll_loop(stdout, &mut state)?;
	terminal::disable_raw_mode()?;
	Ok(backgrounded)
}

/// Inner poll loop with key check
fn run_poll_loop(
	stdout: &mut io::Stdout,
	state: &mut PollState,
) -> io::Result<bool> {
	loop {
		let action = check_progress_key(stdout)?;
		if action != ProgressKeyAction::None {
			return Ok(
				action
					== ProgressKeyAction::Background,
			);
		}
		if !draw_status(stdout, state)? {
			return Ok(false);
		}
	}
}

/// Fetch status and draw, return false if complete
fn draw_status(
	stdout: &mut io::Stdout,
	state: &mut PollState,
) -> io::Result<bool> {
	let status = fetch_docgen_status(state);
	let Some(status) = status else {
		return Ok(true);
	};
	let total = status.total.max(1);
	let (bar_str, progress) = format_progress_bar(
		status.completed, total,
	);
	let spinner =
		SPINNER[state.spin_idx % SPINNER.len()];
	draw_bar_line(
		stdout, spinner, &bar_str, progress,
	)?;
	draw_docgen_stats(
		stdout, status.completed,
		total, &state.start,
	)?;
	stdout.flush()?;
	state.spin_idx += 1;
	Ok(!is_docgen_complete(&status))
}

/// Check if doc generation has finished
fn is_docgen_complete(
	status: &DocGenStatus,
) -> bool {
	status.is_ready
		|| (!status.in_progress
			&& status.completed > 0)
}

/// Fetch doc gen status from daemon
fn fetch_docgen_status(
	state: &mut PollState,
) -> Option<DocGenStatus> {
	match state
		.client
		.doc_gen_status(
			state.project_path.to_string(),
		) {
		Ok(stat) => Some(stat),
		Err(_) => {
			state.spin_idx += 1;
			None
		}
	}
}
