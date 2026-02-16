//! Tests for title bar progress indicator.

use rustean::app::doc_progress::DocProgressState;
use rustean::ui::components::title_progress::{
	build_progress_span,
};

#[test]
fn span_none_when_not_visible() {
	let state = DocProgressState::new();
	assert!(build_progress_span(&state).is_none());
}

#[test]
fn span_some_when_in_progress() {
	let mut state = DocProgressState::new();
	state.update(100, 25, true);
	let span = build_progress_span(&state);
	assert!(span.is_some());
	let text = span.unwrap().content.to_string();
	assert!(text.contains("25%"));
}

#[test]
fn span_shows_done_after_completion() {
	let mut state = DocProgressState::new();
	state.update(100, 50, true);
	state.update(100, 100, false);
	let span = build_progress_span(&state);
	assert!(span.is_some());
	let text = span.unwrap().content.to_string();
	assert!(text.contains("done"));
}

#[test]
fn span_shows_correct_percentage() {
	let mut state = DocProgressState::new();
	state.update(200, 150, true);
	let span = build_progress_span(&state);
	assert!(span.is_some());
	let text = span.unwrap().content.to_string();
	assert!(text.contains("75%"));
}
