//! Tests for DocProgressState.

use rustean::app::doc_progress::DocProgressState;

#[test]
fn new_progress_state_is_zeroed() {
	let state = DocProgressState::new();
	assert_eq!(state.total, 0);
	assert_eq!(state.completed, 0);
	assert!(!state.in_progress);
	assert_eq!(state.percent(), 0);
}

#[test]
fn percent_calculation_mid_progress() {
	let mut state = DocProgressState::new();
	state.update(100, 45, true);
	assert_eq!(state.percent(), 45);
}

#[test]
fn percent_calculation_complete() {
	let mut state = DocProgressState::new();
	state.update(200, 200, false);
	assert_eq!(state.percent(), 100);
}

#[test]
fn percent_zero_total_returns_zero() {
	let state = DocProgressState::new();
	assert_eq!(state.percent(), 0);
}

#[test]
fn is_visible_while_in_progress() {
	let mut state = DocProgressState::new();
	state.update(100, 10, true);
	assert!(state.is_visible());
}

#[test]
fn is_visible_briefly_after_done() {
	let mut state = DocProgressState::new();
	state.update(100, 50, true);
	// transition to done
	state.update(100, 100, false);
	// should be visible right after finishing
	assert!(state.is_visible());
}

#[test]
fn not_visible_when_never_started() {
	let state = DocProgressState::new();
	assert!(!state.is_visible());
}

#[test]
fn should_poll_every_20_ticks() {
	let mut state = DocProgressState::new();
	// first 19 ticks return false
	for _ in 0..19 {
		assert!(!state.should_poll());
	}
	// 20th tick returns true
	assert!(state.should_poll());
	// next 19 false again
	for _ in 0..19 {
		assert!(!state.should_poll());
	}
	assert!(state.should_poll());
}

#[test]
fn update_preserves_running_transition() {
	let mut state = DocProgressState::new();
	state.update(100, 50, true);
	assert!(state.in_progress);

	state.update(100, 100, false);
	assert!(!state.in_progress);
	assert_eq!(state.completed, 100);
}
