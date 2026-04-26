use crossterm::event::{KeyCode, KeyModifiers};
use rustean::app::App;

/// Helper: type a string into the app.
fn type_text(app: &mut App, text: &str) {
	for ch in text.chars() {
		if ch == '\n' {
			// Shift+Enter for newline
			app.handle_key(
				KeyCode::Enter,
				KeyModifiers::SHIFT,
			);
		} else {
			app.handle_key(
				KeyCode::Char(ch),
				KeyModifiers::NONE,
			);
		}
	}
}

#[test]
fn enter_without_agents_sends_directly() {
	// Arrange
	let mut app = App::default();
	type_text(&mut app, "Hello world");

	// Act — Enter sends (no preflight)
	app.handle_key(
		KeyCode::Enter,
		KeyModifiers::NONE,
	);

	// Assert — no preflight, message sent
	assert!(app.preflight().is_none());
	assert_eq!(app.input(), "");
}

#[test]
fn enter_with_agents_shows_preflight() {
	// Arrange
	let mut app = App::default();
	type_text(
		&mut app,
		"Fix bug\n- agent(sonnet): Review",
	);

	// Act — Enter triggers preflight
	app.handle_key(
		KeyCode::Enter,
		KeyModifiers::NONE,
	);

	// Assert — preflight active, input preserved
	assert!(app.preflight().is_some());
	let pf = app.preflight().unwrap();
	assert_eq!(pf.agents.len(), 1);
	assert_eq!(pf.agents[0].model, "sonnet");
	// Input is still there (not cleared)
	assert!(!app.input().is_empty());
}

#[test]
fn esc_cancels_preflight() {
	// Arrange
	let mut app = App::default();
	type_text(
		&mut app,
		"Fix\n- agent(haiku): Test",
	);
	app.handle_key(
		KeyCode::Enter,
		KeyModifiers::NONE,
	);
	assert!(app.preflight().is_some());

	// Act — Esc cancels
	app.handle_key(
		KeyCode::Esc,
		KeyModifiers::NONE,
	);

	// Assert — preflight cleared, input intact
	assert!(app.preflight().is_none());
	assert!(!app.input().is_empty());
}
