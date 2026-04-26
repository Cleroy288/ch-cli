//! Unit tests for App default state.

use rustean::app::App;

/// Default app has empty input and inactive picker
#[test]
fn default_app_has_empty_state() {
    // Arrange + Act
    let app = App::default();

    // Assert
    assert_eq!(app.input(), "");
    assert_eq!(app.cursor_position(), 0);
    assert!(!app.should_quit());
    assert!(!app.picker().is_active());
    assert_eq!(app.file_references().len(), 0);
}

/// Default conversation history is empty
#[test]
fn default_history_is_empty() {
    // Arrange + Act
    let app = App::default();

    // Assert
    assert_eq!(app.history().messages().len(), 0);
}
