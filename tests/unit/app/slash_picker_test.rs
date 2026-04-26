//! Tests for slash picker app-level key handling

use crossterm::event::{KeyCode, KeyModifiers};
use rustean::app::App;
use rustean::picker::PickerMode;

/// Typing / activates picker in SlashCommand mode
#[test]
fn slash_activates_slash_command_picker() {
    // Arrange
    let mut app = App::default();

    // Act
    app.handle_key(
        KeyCode::Char('/'),
        KeyModifiers::NONE,
    );

    // Assert
    assert!(app.picker().is_active());
    assert_eq!(
        *app.picker().mode(),
        PickerMode::SlashCommand
    );
    assert_eq!(app.input(), "/");
}

/// Esc in slash picker deactivates and removes /
#[test]
fn esc_cancels_slash_picker() {
    // Arrange
    let mut app = App::default();
    app.handle_key(
        KeyCode::Char('/'),
        KeyModifiers::NONE,
    );

    // Act
    app.handle_key(KeyCode::Esc, KeyModifiers::NONE);

    // Assert
    assert!(!app.picker().is_active());
    assert!(!app.input().contains('/'));
}

/// Typing in slash picker updates query
#[test]
fn typing_in_slash_picker_updates_query() {
    // Arrange
    let mut app = App::default();
    app.handle_key(
        KeyCode::Char('/'),
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

/// Backspace on empty query cancels slash picker
#[test]
fn backspace_empty_cancels_slash_picker() {
    // Arrange
    let mut app = App::default();
    app.handle_key(
        KeyCode::Char('/'),
        KeyModifiers::NONE,
    );

    // Act
    app.handle_key(
        KeyCode::Backspace,
        KeyModifiers::NONE,
    );

    // Assert
    assert!(!app.picker().is_active());
}

/// Non-matching query auto-dismisses picker
#[test]
fn no_match_dismisses_picker_as_text() {
    // Arrange
    let mut app = App::default();
    press(&mut app, KeyCode::Char('/'));

    // Act — type chars that match nothing
    press(&mut app, KeyCode::Char('z'));

    // Assert — picker closed, text in input
    assert!(!app.picker().is_active());
    assert!(app.input().contains('z'));
}

/// Arrow down in slash picker changes selection
#[test]
fn arrow_down_moves_selection() {
    // Arrange
    let mut app = App::default();
    press(&mut app, KeyCode::Char('/'));
    let initial = app.picker().selected_index();

    // Act
    press(&mut app, KeyCode::Down);

    // Assert
    assert_eq!(
        app.picker().selected_index(),
        initial + 1
    );
}

/// Backspace with query pops a character
#[test]
fn backspace_with_query_pops_char() {
    // Arrange
    let mut app = App::default();
    press(&mut app, KeyCode::Char('/'));
    press(&mut app, KeyCode::Char('m'));
    assert_eq!(app.picker().query(), "m");

    // Act
    press(&mut app, KeyCode::Backspace);

    // Assert — query empty, picker still active
    assert!(app.picker().is_active());
    assert_eq!(app.picker().query(), "");
}

/// Helper: press a key with no modifiers
fn press(app: &mut App, key: KeyCode) {
    app.handle_key(key, KeyModifiers::NONE);
}
