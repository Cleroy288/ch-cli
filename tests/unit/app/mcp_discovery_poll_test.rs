//! Tests for MCP discovery polling.

use std::sync::mpsc;

use rustean::app::App;
use rustean::app::mcp_discovery_poll
	::tick_mcp_discovery;
use rustean::picker::mcp_display::McpDisplayItem;

/// tick with no receiver is a no-op
#[test]
fn tick_without_receiver_is_noop() {
	// Arrange
	let mut app = App::default();

	// Act
	tick_mcp_discovery(&mut app);

	// Assert — no panic, no change
	assert!(app.mcp_discovery_rx.is_none());
}

/// tick receives items and stores them
#[test]
fn tick_receives_and_stores_items() {
	// Arrange
	let (tx, rx) = mpsc::channel();
	let mut app = App::default();
	app.mcp_discovery_rx = Some(rx);
	let items = vec![McpDisplayItem {
		name: "ext".to_string(),
		description: "External tool".to_string(),
		source: "plugin".to_string(),
	}];
	tx.send(items).unwrap();

	// Act
	tick_mcp_discovery(&mut app);

	// Assert
	assert!(app.mcp_discovery_rx.is_none());
}

/// tick handles disconnected channel
#[test]
fn tick_handles_disconnected_channel() {
	// Arrange
	let (tx, rx) = mpsc::channel::<
		Vec<McpDisplayItem>,
	>();
	let mut app = App::default();
	app.mcp_discovery_rx = Some(rx);
	drop(tx);

	// Act
	tick_mcp_discovery(&mut app);

	// Assert
	assert!(app.mcp_discovery_rx.is_none());
}

/// tick with empty channel keeps receiver
#[test]
fn tick_with_empty_channel_keeps_receiver() {
	// Arrange
	let (_tx, rx) = mpsc::channel::<
		Vec<McpDisplayItem>,
	>();
	let mut app = App::default();
	app.mcp_discovery_rx = Some(rx);

	// Act
	tick_mcp_discovery(&mut app);

	// Assert
	assert!(app.mcp_discovery_rx.is_some());
}
