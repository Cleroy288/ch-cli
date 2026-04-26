//! Unit tests for fs::entry — migrated from inline tests

use std::path::PathBuf;

use rustean::domain::{DIR_SYMBOL, FILE_SYMBOL};
use rustean::fs::FsEntry;

/// FsEntry::new sets path, name, and is_dir for a file
#[test]
fn test_new_file() {
    let path = PathBuf::from("/home/user/test.txt");
    let entry = FsEntry::new(path.clone(), false);

    assert_eq!(entry.path, path);
    assert_eq!(entry.name, "test.txt");
    assert!(!entry.is_dir);
}

/// FsEntry::new sets is_dir true for a directory
#[test]
fn test_new_directory() {
    let path = PathBuf::from("/home/user/projects");
    let entry = FsEntry::new(path.clone(), true);

    assert_eq!(entry.path, path);
    assert_eq!(entry.name, "projects");
    assert!(entry.is_dir);
}

/// display_name prefixes files with FILE_SYMBOL
#[test]
fn test_display_name_file() {
    let path = PathBuf::from("test.rs");
    let entry = FsEntry::new(path, false);

    assert_eq!(
        entry.display_name(),
        format!("{} test.rs", FILE_SYMBOL)
    );
}

/// display_name prefixes directories with DIR_SYMBOL
#[test]
fn test_display_name_directory() {
    let path = PathBuf::from("src");
    let entry = FsEntry::new(path, true);

    assert_eq!(
        entry.display_name(),
        format!("{} src", DIR_SYMBOL)
    );
}

/// path_string converts PathBuf to String
#[test]
fn test_path_string() {
    let path = PathBuf::from("src/main.rs");
    let entry = FsEntry::new(path, false);

    assert_eq!(entry.path_string(), "src/main.rs");
}

/// name_only returns the filename without path
#[test]
fn test_name_only() {
    let path =
        PathBuf::from("/home/user/projects/test.rs");
    let entry = FsEntry::new(path, false);

    assert_eq!(entry.name_only(), "test.rs");
}
