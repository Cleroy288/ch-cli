//! Unit tests for fs::scanner — migrated from inline tests

use std::fs;

use tempfile::tempdir;

use rustean::fs::FileScanner;

/// FileScanner::new() creates empty scanner
#[test]
fn test_new() {
    let scanner = FileScanner::new();
    assert_eq!(scanner.entries().len(), 0);
}

/// FileScanner::default() creates empty scanner
#[test]
fn test_default() {
    let scanner = FileScanner::default();
    assert_eq!(scanner.entries().len(), 0);
}

/// scan_directory() indexes files and dirs
#[test]
fn test_scan_directory() {
    let temp_dir = tempdir().unwrap();
    let base = temp_dir.path();

    fs::create_dir(base.join("dir1")).unwrap();
    fs::create_dir(base.join("dir2")).unwrap();
    fs::write(base.join("file1.txt"), "test").unwrap();
    fs::write(base.join("file2.rs"), "code").unwrap();

    let mut scanner = FileScanner::new();
    scanner.scan_directory(base).unwrap();

    let entries = scanner.entries();
    assert_eq!(entries.len(), 4);

    // Directories come first
    assert!(entries[0].is_dir);
    assert!(entries[1].is_dir);
    assert!(!entries[2].is_dir);
    assert!(!entries[3].is_dir);
}

/// scan_directory() handles nested directories
#[test]
fn test_scan_directory_recursive() {
    let temp_dir = tempdir().unwrap();
    let base = temp_dir.path();

    let nested = base.join("dir1").join("dir2");
    fs::create_dir_all(&nested).unwrap();
    fs::write(nested.join("file.txt"), "nested").unwrap();

    let mut scanner = FileScanner::new();
    scanner.scan_directory(base).unwrap();

    let entries = scanner.entries();
    // dir1, dir2, file.txt
    assert_eq!(entries.len(), 3);
}

/// entries() returns all scanned entries
#[test]
fn test_entries() {
    let temp_dir = tempdir().unwrap();
    let base = temp_dir.path();

    fs::create_dir(base.join("test_dir")).unwrap();
    fs::write(base.join("test.txt"), "data").unwrap();

    let mut scanner = FileScanner::new();
    scanner.scan_directory(base).unwrap();

    let entries = scanner.entries();
    assert_eq!(entries.len(), 2);
}
