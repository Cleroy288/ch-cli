//! Unit tests for app::App — migrated from inline tests

use rustean::app::App;

/// Test App::default() creates app with empty initial state
#[test]
fn test_new() {
    let app = App::default();

    assert_eq!(app.input(), "");
    assert_eq!(app.cursor_position(), 0);
    assert!(!app.should_quit());
    assert!(!app.picker().is_active());
    assert_eq!(app.file_references().len(), 0);
}

/// Test App::default() creates app with empty initial state
#[test]
fn test_default() {
    let app = App::default();

    assert_eq!(app.input(), "");
    assert_eq!(app.cursor_position(), 0);
    assert!(!app.should_quit());
}

/// Test input() getter returns current input text
#[test]
fn test_input_getter() {
    let app = App::default();
    assert_eq!(app.input(), "");
}

/// Test cursor_position() getter returns initial position
#[test]
fn test_cursor_position_getter() {
    let app = App::default();
    assert_eq!(app.cursor_position(), 0);
}

/// Test should_quit() getter returns false initially
#[test]
fn test_should_quit_getter() {
    let app = App::default();
    assert!(!app.should_quit());
}

/// Test picker() getter returns reference to Picker
#[test]
fn test_picker_getter() {
    let app = App::default();
    let picker = app.picker();
    assert!(!picker.is_active());
}

/// Test file_references() getter returns empty slice
#[test]
fn test_file_references_getter() {
    let app = App::default();
    assert_eq!(app.file_references().len(), 0);
}

/// Test history() getter returns ConversationHistory
#[test]
fn test_history_getter() {
    let app = App::default();
    let history = app.history();
    assert_eq!(history.messages().len(), 0);
}
