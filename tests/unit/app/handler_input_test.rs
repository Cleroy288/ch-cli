//! Tests for input key handlers

use crossterm::event::{KeyCode, KeyModifiers};
use rustean::app::App;

/// Typing a character inserts it and moves cursor
#[test]
fn char_input_inserts_and_moves_cursor() {
    // Arrange
    let mut app = App::default();

    // Act
    app.handle_key(KeyCode::Char('h'), KeyModifiers::NONE);

    // Assert
    assert_eq!(app.input(), "h");
    assert_eq!(app.cursor_position(), 1);
}

/// Backspace removes character before cursor
#[test]
fn backspace_removes_previous_char() {
    // Arrange
    let mut app = App::default();
    app.handle_key(KeyCode::Char('a'), KeyModifiers::NONE);
    app.handle_key(KeyCode::Char('b'), KeyModifiers::NONE);

    // Act
    app.handle_key(
        KeyCode::Backspace,
        KeyModifiers::NONE,
    );

    // Assert
    assert_eq!(app.input(), "a");
    assert_eq!(app.cursor_position(), 1);
}

/// Backspace at position 0 does nothing
#[test]
fn backspace_at_start_is_noop() {
    // Arrange
    let mut app = App::default();

    // Act
    app.handle_key(
        KeyCode::Backspace,
        KeyModifiers::NONE,
    );

    // Assert
    assert_eq!(app.input(), "");
    assert_eq!(app.cursor_position(), 0);
}

/// Delete key removes char at cursor position
#[test]
fn delete_removes_char_at_cursor() {
    // Arrange
    let mut app = App::default();
    app.handle_key(KeyCode::Char('a'), KeyModifiers::NONE);
    app.handle_key(KeyCode::Char('b'), KeyModifiers::NONE);
    app.handle_key(KeyCode::Home, KeyModifiers::NONE);

    // Act
    app.handle_key(KeyCode::Delete, KeyModifiers::NONE);

    // Assert
    assert_eq!(app.input(), "b");
}

/// Home key moves cursor to start of input
#[test]
fn home_moves_cursor_to_start() {
    // Arrange
    let mut app = App::default();
    app.handle_key(KeyCode::Char('a'), KeyModifiers::NONE);
    app.handle_key(KeyCode::Char('b'), KeyModifiers::NONE);

    // Act
    app.handle_key(KeyCode::Home, KeyModifiers::NONE);

    // Assert
    assert_eq!(app.cursor_position(), 0);
}
