//! Tests for ( symbol picker trigger detection
//! and app quit behavior

use crossterm::event::{KeyCode, KeyModifiers};
use rustean::app::App;

/// ( typed without preceding file ref does not open
/// the symbol picker
#[test]
fn paren_without_file_ref_no_picker() {
    // Arrange
    let mut app = App::default();

    // Act
    app.handle_key(
        KeyCode::Char('('),
        KeyModifiers::NONE,
    );

    // Assert
    assert!(!app.picker().is_active());
}

/// ( after regular text does not trigger picker
#[test]
fn paren_after_text_no_trigger() {
    // Arrange
    let mut app = App::default();
    app.handle_key(KeyCode::Char('h'), KeyModifiers::NONE);
    app.handle_key(KeyCode::Char('i'), KeyModifiers::NONE);

    // Act
    app.handle_key(
        KeyCode::Char('('),
        KeyModifiers::NONE,
    );

    // Assert
    assert!(!app.picker().is_active());
    assert_eq!(app.input(), "hi(");
}

/// Ctrl+C sets should_quit and returns true
#[test]
fn ctrl_c_quits_app() {
    // Arrange
    let mut app = App::default();

    // Act
    let quit = app.handle_key(
        KeyCode::Char('c'),
        KeyModifiers::CONTROL,
    );

    // Assert
    assert!(quit);
    assert!(app.should_quit());
}

/// Esc in input mode sets should_quit
#[test]
fn esc_in_input_mode_quits() {
    // Arrange
    let mut app = App::default();

    // Act
    let quit = app.handle_key(
        KeyCode::Esc,
        KeyModifiers::NONE,
    );

    // Assert
    assert!(quit);
    assert!(app.should_quit());
}
