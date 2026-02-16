//! Tests for handle_enter message creation

use crossterm::event::{KeyCode, KeyModifiers};
use rustean::app::App;

/// Enter with text creates message in history
#[test]
fn enter_with_text_creates_message() {
    // Arrange
    let mut app = App::default();
    app.handle_key(KeyCode::Char('h'), KeyModifiers::NONE);
    app.handle_key(KeyCode::Char('i'), KeyModifiers::NONE);

    // Act
    app.handle_key(KeyCode::Enter, KeyModifiers::NONE);

    // Assert
    assert_eq!(app.history().messages().len(), 1);
    assert_eq!(app.input(), "");
}

/// Enter with empty input creates no message
#[test]
fn enter_empty_creates_no_message() {
    // Arrange
    let mut app = App::default();

    // Act
    app.handle_key(KeyCode::Enter, KeyModifiers::NONE);

    // Assert
    assert_eq!(app.history().messages().len(), 0);
}

/// Enter clears input and resets cursor
#[test]
fn enter_clears_input_and_cursor() {
    // Arrange
    let mut app = App::default();
    app.handle_key(KeyCode::Char('x'), KeyModifiers::NONE);

    // Act
    app.handle_key(KeyCode::Enter, KeyModifiers::NONE);

    // Assert
    assert_eq!(app.input(), "");
    assert_eq!(app.cursor_position(), 0);
}

/// Enter clears file refs and symbol selectors
#[test]
fn enter_clears_file_refs_and_selectors() {
    // Arrange
    let mut app = App::default();
    app.handle_key(KeyCode::Char('x'), KeyModifiers::NONE);

    // Act
    app.handle_key(KeyCode::Enter, KeyModifiers::NONE);

    // Assert
    assert_eq!(app.file_references().len(), 0);
    assert_eq!(app.symbol_selectors().len(), 0);
}
