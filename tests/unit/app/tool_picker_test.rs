//! Tests for tool picker activation and navigation.

use crossterm::event::{KeyCode, KeyModifiers};
use rustean::app::App;
use rustean::picker::PickerMode;

/// '#' activates Tools mode
#[test]
fn hash_activates_tools_mode() {
    // Arrange
    let mut app = App::default();

    // Act
    app.handle_key(
        KeyCode::Char('#'),
        KeyModifiers::NONE,
    );

    // Assert
    assert!(matches!(
        app.picker().mode(),
        PickerMode::Tools
    ));
}

/// Esc in Tools mode cancels picker
#[test]
fn esc_in_tools_cancels_picker() {
    // Arrange
    let mut app = App::default();
    app.handle_key(
        KeyCode::Char('#'),
        KeyModifiers::NONE,
    );

    // Act
    app.handle_key(
        KeyCode::Esc,
        KeyModifiers::NONE,
    );

    // Assert
    assert!(matches!(
        app.picker().mode(),
        PickerMode::Inactive
    ));
}

/// Backspace on empty query cancels picker
#[test]
fn backspace_empty_cancels_tools() {
    // Arrange
    let mut app = App::default();
    app.handle_key(
        KeyCode::Char('#'),
        KeyModifiers::NONE,
    );

    // Act
    app.handle_key(
        KeyCode::Backspace,
        KeyModifiers::NONE,
    );

    // Assert
    assert!(matches!(
        app.picker().mode(),
        PickerMode::Inactive
    ));
}

/// Typing filters tools list
#[test]
fn typing_in_tools_updates_query() {
    // Arrange
    let mut app = App::default();
    app.handle_key(
        KeyCode::Char('#'),
        KeyModifiers::NONE,
    );

    // Act
    app.handle_key(
        KeyCode::Char('b'),
        KeyModifiers::NONE,
    );

    // Assert
    assert_eq!(app.picker().query(), "b");
}

/// Down arrow navigates tools list
#[test]
fn down_moves_selection() {
    // Arrange
    let mut app = App::default();
    app.handle_key(
        KeyCode::Char('#'),
        KeyModifiers::NONE,
    );

    // Act
    app.handle_key(
        KeyCode::Down,
        KeyModifiers::NONE,
    );

    // Assert
    assert_eq!(app.picker().selected_index(), 1);
}
