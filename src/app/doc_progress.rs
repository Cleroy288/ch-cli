//! Doc generation progress state for TUI.
//!
//! Tracks background doc generation status and controls
//! polling frequency from the daemon.

use std::time::Instant;

/// Poll daemon every 20 ticks (2s at 100ms tick rate)
const POLL_INTERVAL: usize = 20;

/// Hide "done" indicator after 10 seconds
const DONE_VISIBLE_SECS: u64 = 10;

/// Background doc generation progress state.
pub struct DocProgressState {
	/// total entries to process
	pub total: usize,
	/// completed entries so far
	pub completed: usize,
	/// whether generation is running
	pub in_progress: bool,
	/// tick counter for poll timing
	tick: usize,
	/// when generation finished (for auto-hide)
	done_since: Option<Instant>,
}

impl DocProgressState {
	/// Create a new progress state.
	pub fn new() -> Self {
		Self {
			total: 0,
			completed: 0,
			in_progress: false,
			tick: 0,
			done_since: None,
		}
	}

	/// Update state from daemon status response.
	pub fn update(
		&mut self,
		total: usize,
		completed: usize,
		in_progress: bool,
	) {
		self.total = total;
		self.completed = completed;
		let was_running = self.in_progress;
		self.in_progress = in_progress;
		if was_running && !in_progress {
			self.done_since = Some(Instant::now());
		}
	}

	/// Check if it's time to poll the daemon.
	///
	/// Increments tick counter; returns true every
	/// POLL_INTERVAL ticks.
	pub fn should_poll(&mut self) -> bool {
		self.tick += 1;
		self.tick.is_multiple_of(POLL_INTERVAL)
	}

	/// Get completion percentage (0-100).
	pub fn percent(&self) -> u8 {
		if self.total == 0 {
			return 0;
		}
		let pct = (self.completed * 100) / self.total;
		pct.min(100) as u8
	}

	/// Check if the progress indicator should be visible.
	///
	/// Visible while running, or for DONE_VISIBLE_SECS
	/// after completion. Hidden otherwise.
	pub fn is_visible(&self) -> bool {
		if self.in_progress {
			return true;
		}
		if let Some(done_at) = self.done_since {
			return done_at.elapsed().as_secs()
				< DONE_VISIBLE_SECS;
		}
		false
	}
}

impl Default for DocProgressState {
	fn default() -> Self {
		Self::new()
	}
}
