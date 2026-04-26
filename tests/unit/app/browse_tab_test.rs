//! Tests for Tab key in browse mode.
//!
//! Tab inserts a reference for the selected entry.
//! Folders get a trailing `/` in display name.

use std::path::PathBuf;

use crossterm::event::{KeyCode, KeyModifiers};
use rustean::app::App;
use rustean::fs::{FileCache, FsEntry};

/// Build a cache with one dir and one file at root.
fn make_test_cache() -> FileCache {
    let entries = vec![
        FsEntry::new(PathBuf::from("./src"), true),
        FsEntry::new(
            PathBuf::from("./main.rs"),
            false,
        ),
    ];
    FileCache::with_entries(entries)
}

/// Tab on a folder inserts dir ref with trailing /
#[test]
fn tab_on_folder_inserts_dir_ref() {
    // Arrange
    let cache = make_test_cache();
    let mut app = App::new(cache);
    app.handle_key(
        KeyCode::Char('@'),
        KeyModifiers::NONE,
    );

    // Act — first entry is dir "src"
    app.handle_key(KeyCode::Tab, KeyModifiers::NONE);

    // Assert
    assert!(!app.picker().is_active());
    assert!(app.input().contains("src/"));
    assert_eq!(app.file_references().len(), 1);
    let fref = &app.file_references()[0];
    assert!(fref.is_dir);
}

/// Tab on a file inserts file ref (no trailing /)
#[test]
fn tab_on_file_inserts_file_ref() {
    // Arrange
    let cache = make_test_cache();
    let mut app = App::new(cache);
    app.handle_key(
        KeyCode::Char('@'),
        KeyModifiers::NONE,
    );
    // Move down to file entry
    app.handle_key(KeyCode::Down, KeyModifiers::NONE);

    // Act
    app.handle_key(KeyCode::Tab, KeyModifiers::NONE);

    // Assert
    assert!(!app.picker().is_active());
    assert!(app.input().contains("main.rs"));
    assert!(!app.input().contains("main.rs/"));
    assert_eq!(app.file_references().len(), 1);
    let fref = &app.file_references()[0];
    assert!(!fref.is_dir);
}

/// Tab with no entries does nothing
#[test]
fn tab_empty_picker_does_nothing() {
    // Arrange
    let mut app = App::default();
    app.handle_key(
        KeyCode::Char('@'),
        KeyModifiers::NONE,
    );

    // Act
    app.handle_key(KeyCode::Tab, KeyModifiers::NONE);

    // Assert — picker stays active, no crash
    assert!(app.picker().is_active());
    assert_eq!(app.file_references().len(), 0);
}
