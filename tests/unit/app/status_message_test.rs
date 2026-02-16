//! Tests for transient status message behavior

use crossterm::event::{KeyCode, KeyModifiers};
use rustean::app::App;

/// Status message starts as None
#[test]
fn status_message_starts_none() {
    // Arrange + Act
    let app = App::default();

    // Assert
    assert!(app.status_message().is_none());
}

/// Any key press clears the status message
#[test]
fn key_press_clears_status_message() {
    // Arrange
    let mut app = App::default();
    app.set_status_message(
        Some("test message".to_string()),
    );
    assert!(app.status_message().is_some());

    // Act
    app.handle_key(
        KeyCode::Char('a'),
        KeyModifiers::NONE,
    );

    // Assert
    assert!(app.status_message().is_none());
}

/// Esc also clears status before quitting
#[test]
fn esc_clears_status_message() {
    // Arrange
    let mut app = App::default();
    app.set_status_message(
        Some("test message".to_string()),
    );

    // Act
    app.handle_key(KeyCode::Esc, KeyModifiers::NONE);

    // Assert
    assert!(app.status_message().is_none());
}

/// Set and get status message round-trip
#[test]
fn set_get_status_message() {
    // Arrange
    let mut app = App::default();

    // Act
    app.set_status_message(
        Some("No symbols available".to_string()),
    );

    // Assert
    assert_eq!(
        app.status_message(),
        Some("No symbols available"),
    );
}
