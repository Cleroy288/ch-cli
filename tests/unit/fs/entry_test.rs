//! Unit tests for fs::entry — migrated from inline tests

use std::path::PathBuf;

use ch_cli::domain::{DIR_SYMBOL, FILE_SYMBOL};
use ch_cli::fs::FsEntry;

/// Test FsEntry::new() creates entry with correct fields
#[test]
fn test_new_file() {
    let path = PathBuf::from("/home/user/test.txt");
    let entry = FsEntry::new(path.clone(), false);

    assert_eq!(entry.path, path);
    assert_eq!(entry.name, "test.txt");
    assert!(!entry.is_dir);
}

/// Test FsEntry::new() creates directory entry correctly
#[test]
fn test_new_directory() {
    let path = PathBuf::from("/home/user/projects");
    let entry = FsEntry::new(path.clone(), true);

    assert_eq!(entry.path, path);
    assert_eq!(entry.name, "projects");
    assert!(entry.is_dir);
}

/// Test display_name() returns file with FILE_SYMBOL prefix
#[test]
fn test_display_name_file() {
    let path = PathBuf::from("test.rs");
    let entry = FsEntry::new(path, false);

    assert_eq!(
        entry.display_name(),
        format!("{} test.rs", FILE_SYMBOL)
    );
}

/// Test display_name() returns dir with DIR_SYMBOL prefix
#[test]
fn test_display_name_directory() {
    let path = PathBuf::from("src");
    let entry = FsEntry::new(path, true);

    assert_eq!(
        entry.display_name(),
        format!("{} src", DIR_SYMBOL)
    );
}

/// Test path_string() converts PathBuf to String
#[test]
fn test_path_string() {
    let path = PathBuf::from("src/main.rs");
    let entry = FsEntry::new(path, false);

    assert_eq!(entry.path_string(), "src/main.rs");
}

/// Test name_only() returns just the filename
#[test]
fn test_name_only() {
    let path =
        PathBuf::from("/home/user/projects/test.rs");
    let entry = FsEntry::new(path, false);

    assert_eq!(entry.name_only(), "test.rs");
}
