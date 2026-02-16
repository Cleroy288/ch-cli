//! Unit tests for fs::scanner_filters — migrated from inline tests

use std::fs;

use tempfile::tempdir;

use rustean::fs::FileScanner;

/// directories() returns only directory entries
#[test]
fn test_directories() {
    let temp_dir = tempdir().unwrap();
    let base = temp_dir.path();

    fs::create_dir(base.join("dir1")).unwrap();
    fs::create_dir(base.join("dir2")).unwrap();
    fs::write(base.join("file.txt"), "test").unwrap();

    let mut scanner = FileScanner::new();
    scanner.scan_directory(base).unwrap();

    let dirs = scanner.directories();
    assert_eq!(dirs.len(), 2);
    assert!(dirs.iter().all(|e| e.is_dir));
}

/// files() returns only file entries
#[test]
fn test_files() {
    let temp_dir = tempdir().unwrap();
    let base = temp_dir.path();

    fs::create_dir(base.join("dir1")).unwrap();
    fs::write(base.join("file1.txt"), "test").unwrap();
    fs::write(base.join("file2.rs"), "code").unwrap();

    let mut scanner = FileScanner::new();
    scanner.scan_directory(base).unwrap();

    let files = scanner.files();
    assert_eq!(files.len(), 2);
    assert!(files.iter().all(|e| !e.is_dir));
}

/// search() filters entries by name
#[test]
fn test_search_by_name() {
    let temp_dir = tempdir().unwrap();
    let base = temp_dir.path();

    fs::write(base.join("Test.txt"), "data").unwrap();
    fs::write(base.join("other.rs"), "code").unwrap();

    let mut scanner = FileScanner::new();
    scanner.scan_directory(base).unwrap();

    let results = scanner.search("test", false, false);
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].name, "Test.txt");
}

/// search() with empty query returns all entries
#[test]
fn test_search_empty_query() {
    let temp_dir = tempdir().unwrap();
    let base = temp_dir.path();

    fs::create_dir(base.join("dir")).unwrap();
    fs::write(base.join("file.txt"), "data").unwrap();

    let mut scanner = FileScanner::new();
    scanner.scan_directory(base).unwrap();

    let results = scanner.search("", false, false);
    assert_eq!(results.len(), 2);
}

/// search() with files_only filters directories
#[test]
fn test_search_files_only() {
    let temp_dir = tempdir().unwrap();
    let base = temp_dir.path();

    fs::create_dir(base.join("test_dir")).unwrap();
    fs::write(base.join("test.txt"), "data").unwrap();

    let mut scanner = FileScanner::new();
    scanner.scan_directory(base).unwrap();

    let results = scanner.search("test", true, false);
    assert_eq!(results.len(), 1);
    assert!(!results[0].is_dir);
}
