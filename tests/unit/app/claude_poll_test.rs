//! Tests for Claude response polling.

use std::sync::mpsc;

use rustean::app::App;
use rustean::app::claude_poll::tick_claude_response;
use rustean::domain::claude::{
	ClaudeResponse, ClaudeUsage,
};
use rustean::domain::errors::ClaudeError;

/// Helper: build a test ClaudeResponse
fn make_response(text: &str) -> ClaudeResponse {
	ClaudeResponse {
		result: text.to_string(),
		session_id: "test-sess".to_string(),
		is_error: false,
		num_turns: 1,
		cost_usd: Some(0.001),
		duration_ms: 500,
		usage: ClaudeUsage {
			input_tokens: 10,
			output_tokens: 5,
			cache_read_tokens: 0,
			cache_creation_tokens: 0,
		},
	}
}

/// Recv ok stores response and clears receiver
#[test]
fn tick_recv_ok_stores_response() {
	// Arrange
	let mut app = App::default();
	let (tx, rx) = mpsc::channel();
	app.set_claude_rx(Some(rx));
	tx.send(Ok(make_response("hello"))).unwrap();

	// Act
	tick_claude_response(&mut app);

	// Assert
	assert!(app.last_claude_response().is_some());
	assert_eq!(
		app.last_claude_response().unwrap().result,
		"hello",
	);
	assert!(!app.is_claude_loading());
}

/// Recv error sets status message
#[test]
fn tick_recv_err_sets_status_message() {
	// Arrange
	let mut app = App::default();
	let (tx, rx) = mpsc::channel();
	app.set_claude_rx(Some(rx));
	tx.send(Err(ClaudeError::NotInstalled)).unwrap();

	// Act
	tick_claude_response(&mut app);

	// Assert
	assert!(app.status_message().is_some());
	assert!(!app.is_claude_loading());
}

/// Empty channel keeps loading state
#[test]
fn tick_empty_channel_stays_loading() {
	// Arrange
	let mut app = App::default();
	let (_tx, rx) = mpsc::channel::<
		Result<ClaudeResponse, ClaudeError>,
	>();
	app.set_claude_rx(Some(rx));

	// Act
	tick_claude_response(&mut app);

	// Assert
	assert!(app.is_claude_loading());
}

/// Disconnected channel clears receiver
#[test]
fn tick_disconnected_clears_receiver() {
	// Arrange
	let mut app = App::default();
	let (tx, rx) = mpsc::channel::<
		Result<ClaudeResponse, ClaudeError>,
	>();
	app.set_claude_rx(Some(rx));
	drop(tx); // disconnect

	// Act
	tick_claude_response(&mut app);

	// Assert
	assert!(!app.is_claude_loading());
}
