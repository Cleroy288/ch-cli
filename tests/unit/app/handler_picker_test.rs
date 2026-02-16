//! Tests for picker handler key routing

use crossterm::event::{KeyCode, KeyModifiers};
use rustean::app::App;
use rustean::picker::PickerMode;

/// Typing @ activates picker in Browse mode
#[test]
fn at_activates_browse_picker() {
    // Arrange
    let mut app = App::default();

    // Act
    app.handle_key(
        KeyCode::Char('@'),
        KeyModifiers::NONE,
    );

    // Assert
    assert!(app.picker().is_active());
    assert!(matches!(
        app.picker().mode(),
        PickerMode::Browse { .. }
    ));
}

/// Esc in picker deactivates it and removes @
#[test]
fn esc_cancels_picker_and_removes_at() {
    // Arrange
    let mut app = App::default();
    app.handle_key(
        KeyCode::Char('@'),
        KeyModifiers::NONE,
    );
    assert!(app.picker().is_active());

    // Act
    app.handle_key(KeyCode::Esc, KeyModifiers::NONE);

    // Assert
    assert!(!app.picker().is_active());
    assert!(!app.input().contains('@'));
}

/// Typing in picker updates the query string
#[test]
fn typing_in_picker_updates_query() {
    // Arrange
    let mut app = App::default();
    app.handle_key(
        KeyCode::Char('@'),
        KeyModifiers::NONE,
    );

    // Act
    app.handle_key(
        KeyCode::Char('m'),
        KeyModifiers::NONE,
    );

    // Assert
    assert_eq!(app.picker().query(), "m");
}

/// Backspace in picker with empty query cancels
#[test]
fn backspace_empty_query_cancels_picker() {
    // Arrange
    let mut app = App::default();
    app.handle_key(
        KeyCode::Char('@'),
        KeyModifiers::NONE,
    );
    assert!(app.picker().is_active());

    // Act
    app.handle_key(
        KeyCode::Backspace,
        KeyModifiers::NONE,
    );

    // Assert
    assert!(!app.picker().is_active());
}

/// Backspace in picker with query pops last char
#[test]
fn backspace_pops_last_query_char() {
    // Arrange
    let mut app = App::default();
    app.handle_key(
        KeyCode::Char('@'),
        KeyModifiers::NONE,
    );
    app.handle_key(
        KeyCode::Char('a'),
        KeyModifiers::NONE,
    );
    app.handle_key(
        KeyCode::Char('b'),
        KeyModifiers::NONE,
    );
    assert_eq!(app.picker().query(), "ab");

    // Act
    app.handle_key(
        KeyCode::Backspace,
        KeyModifiers::NONE,
    );

    // Assert
    assert_eq!(app.picker().query(), "a");
}
