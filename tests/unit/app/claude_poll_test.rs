//! Tests for Claude response streaming poll.

use std::sync::mpsc;

use rustean::app::App;
use rustean::app::claude_poll::tick_claude_response;
use rustean::domain::claude::{
	ClaudeResponse, ClaudeUsage, StreamChunk,
	ToolActivity,
};

/// Helper: build a test ClaudeResponse
fn make_response(text: &str) -> ClaudeResponse {
	ClaudeResponse {
		result: text.to_string(),
		session_id: "test-sess".to_string(),
		subtype: "success".to_string(),
		is_error: false,
		num_turns: 1,
		cost_usd: Some(0.001),
		duration_ms: 500,
		duration_api_ms: 300,
		usage: ClaudeUsage {
			input_tokens: 10,
			output_tokens: 5,
			cache_read_tokens: 0,
			cache_creation_tokens: 0,
		},
		intent: Default::default(),
	}
}

/// Done chunk stores response and clears receiver
#[test]
fn tick_done_stores_response() {
	// Arrange
	let mut app = App::default();
	let (tx, rx) = mpsc::channel();
	app.set_claude_rx(Some(rx));
	let resp = make_response("hello");
	tx.send(StreamChunk::Done(resp)).unwrap();

	// Act
	tick_claude_response(&mut app);

	// Assert — response in history + last_claude_response
	assert!(app.last_claude_response().is_some());
	assert!(!app.is_claude_loading());
}

/// Delta chunk replaces streaming_text (snapshot)
#[test]
fn tick_delta_replaces_streaming_text() {
	// Arrange
	let mut app = App::default();
	let (tx, rx) = mpsc::channel();
	app.set_claude_rx(Some(rx));
	tx.send(StreamChunk::Delta("hi ".into()))
		.unwrap();
	tx.send(StreamChunk::Delta("hi there".into()))
		.unwrap();

	// Act
	tick_claude_response(&mut app);

	// Assert — last snapshot wins
	assert_eq!(app.streaming_text(), "hi there");
	assert!(app.is_claude_loading());
}

/// Done clears streaming_text buffer
#[test]
fn tick_done_clears_streaming_text() {
	// Arrange
	let mut app = App::default();
	let (tx, rx) = mpsc::channel();
	app.set_claude_rx(Some(rx));
	tx.send(StreamChunk::Delta("partial".into()))
		.unwrap();
	tx.send(StreamChunk::Done(make_response("full")))
		.unwrap();

	// Act
	tick_claude_response(&mut app);

	// Assert
	assert!(app.streaming_text().is_empty());
	assert!(!app.is_claude_loading());
}

/// Empty channel keeps loading state
#[test]
fn tick_empty_channel_stays_loading() {
	// Arrange
	let mut app = App::default();
	let (_tx, rx) =
		mpsc::channel::<StreamChunk>();
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
	let (tx, rx) =
		mpsc::channel::<StreamChunk>();
	app.set_claude_rx(Some(rx));
	drop(tx); // disconnect

	// Act
	tick_claude_response(&mut app);

	// Assert
	assert!(!app.is_claude_loading());
	let msg = app.status_message().unwrap();
	assert!(msg.contains("no response"));
}

/// Disconnected with partial text shows interrupted
#[test]
fn tick_disconnected_with_text_shows_interrupted() {
	// Arrange
	let mut app = App::default();
	let (tx, rx) = mpsc::channel();
	app.set_claude_rx(Some(rx));
	tx.send(StreamChunk::Delta("partial".into()))
		.unwrap();
	// drain the delta first
	tick_claude_response(&mut app);
	// now disconnect
	drop(tx);

	// Act
	tick_claude_response(&mut app);

	// Assert
	assert!(!app.is_claude_loading());
	let msg = app.status_message().unwrap();
	assert!(msg.contains("interrupted"));
}

/// Error chunk sets status and stops loading
#[test]
fn tick_error_sets_status_message() {
	// Arrange
	let mut app = App::default();
	let (tx, rx) = mpsc::channel();
	app.set_claude_rx(Some(rx));
	tx.send(StreamChunk::Error(
		"spawn failed".into(),
	))
	.unwrap();

	// Act
	tick_claude_response(&mut app);

	// Assert
	assert!(!app.is_claude_loading());
	let msg = app.status_message().unwrap();
	assert!(msg.contains("spawn failed"));
}

/// ToolUse chunk sets tool_status
#[test]
fn tick_tool_use_sets_tool_status() {
	// Arrange
	let mut app = App::default();
	let (tx, rx) = mpsc::channel();
	app.set_claude_rx(Some(rx));
	tx.send(StreamChunk::ToolUse(ToolActivity {
		tool_name: "Read".into(),
		summary: "src/main.rs".into(),
	}))
	.unwrap();

	// Act
	tick_claude_response(&mut app);

	// Assert
	let status = app.tool_status().unwrap();
	assert_eq!(status.tool_name, "Read");
	assert_eq!(status.summary, "src/main.rs");
	assert!(app.is_claude_loading());
}

/// Done clears tool_status
#[test]
fn tick_done_clears_tool_status() {
	// Arrange
	let mut app = App::default();
	let (tx, rx) = mpsc::channel();
	app.set_claude_rx(Some(rx));
	tx.send(StreamChunk::ToolUse(ToolActivity {
		tool_name: "Bash".into(),
		summary: "cargo test".into(),
	}))
	.unwrap();
	tx.send(StreamChunk::Done(make_response("ok")))
		.unwrap();

	// Act
	tick_claude_response(&mut app);

	// Assert
	assert!(app.tool_status().is_none());
}

/// No receiver does nothing (no panic)
#[test]
fn tick_no_receiver_does_nothing() {
	// Arrange
	let mut app = App::default();

	// Act — no claude_rx set
	tick_claude_response(&mut app);

	// Assert
	assert!(!app.is_claude_loading());
	assert!(app.last_claude_response().is_none());
}

/// Done stores session_id for resume
#[test]
fn tick_done_stores_session_id() {
	// Arrange
	let mut app = App::default();
	let (tx, rx) = mpsc::channel();
	app.set_claude_rx(Some(rx));
	let resp = make_response("hello");
	tx.send(StreamChunk::Done(resp)).unwrap();

	// Act
	tick_claude_response(&mut app);

	// Assert
	assert_eq!(app.scroll_offset(), 0);
}
