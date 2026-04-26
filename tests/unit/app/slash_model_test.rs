//! Tests for /model slash command via picker flow.
//!
//! The picker intercepts '/' at start of input,
//! so all tests use the picker-based flow.

use crossterm::event::{KeyCode, KeyModifiers};
use rustean::app::App;

/// Helper: press a key with no modifiers
fn press(app: &mut App, key: KeyCode) {
    app.handle_key(key, KeyModifiers::NONE);
}

/// Helper: type a string char by char
fn type_str(app: &mut App, text: &str) {
    for ch in text.chars() {
        press(app, KeyCode::Char(ch));
    }
}

/// /model opus via picker sets model name
#[test]
fn model_opus_via_picker() {
    // Arrange — / opens picker, m filters to model
    let mut app = App::default();
    type_str(&mut app, "/m");
    press(&mut app, KeyCode::Enter);
    // SlashArg: o filters to opus
    type_str(&mut app, "o");
    press(&mut app, KeyCode::Enter);

    // Act — Enter executes /model opus
    press(&mut app, KeyCode::Enter);

    // Assert
    assert_eq!(app.model_name(), "opus");
    let status = app.status_message().unwrap();
    assert!(status.contains("opus"));
}

/// /model haiku via picker sets model name
#[test]
fn model_haiku_via_picker() {
    // Arrange
    let mut app = App::default();
    type_str(&mut app, "/m");
    press(&mut app, KeyCode::Enter);
    type_str(&mut app, "h");
    press(&mut app, KeyCode::Enter);

    // Act
    press(&mut app, KeyCode::Enter);

    // Assert
    assert_eq!(app.model_name(), "haiku");
}

/// /model sonnet via picker (default selection)
#[test]
fn model_sonnet_via_picker() {
    // Arrange
    let mut app = App::default();
    type_str(&mut app, "/m");
    press(&mut app, KeyCode::Enter);
    // s filters to sonnet
    type_str(&mut app, "s");
    press(&mut app, KeyCode::Enter);

    // Act
    press(&mut app, KeyCode::Enter);

    // Assert
    assert_eq!(app.model_name(), "sonnet");
}

/// /new via picker clears conversation
#[test]
fn new_via_picker_resets_state() {
    // Arrange
    let mut app = App::default();
    type_str(&mut app, "/n");
    // Enter selects "new"
    press(&mut app, KeyCode::Enter);

    // Act — Enter executes /new
    press(&mut app, KeyCode::Enter);

    // Assert — status shows new conversation
    let status = app.status_message().unwrap();
    assert!(status.contains("New conversation"));
}
