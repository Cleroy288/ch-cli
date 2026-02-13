//! Unit tests for startup::checks — migrated from inline tests

use std::fs;

use rustean::startup::{
    analyze_codebase, check_index_exists,
    detect_codebase_changes,
};

/// check_index_exists returns false when no index exists
#[test]
fn test_check_index_exists_no_index() {
    let temp_dir =
        std::env::temp_dir().join("test-no-index");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir)
        .expect("Failed to create temp dir");

    let original_dir = std::env::current_dir()
        .expect("Failed to get current dir");
    std::env::set_current_dir(&temp_dir)
        .expect("Failed to change dir");

    let result = check_index_exists();

    std::env::set_current_dir(&original_dir)
        .expect("Failed to restore dir");

    let _ = fs::remove_dir_all(&temp_dir);

    assert!(!result);
}

/// check_index_exists returns true when index exists
#[test]
fn test_check_index_exists_with_index() {
    let temp_dir = std::env::temp_dir()
        .join("test-with-index-v2");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir)
        .expect("Failed to create temp dir");
    let index_dir = temp_dir.join(".rustean-index");
    fs::create_dir_all(&index_dir)
        .expect("Failed to create index dir");

    let state_file = index_dir.join("state.json");
    fs::write(
        &state_file,
        r#"{"version": "1.0", "files": []}"#,
    )
    .expect("Failed to write state file");

    let original_dir = std::env::current_dir()
        .expect("Failed to get current dir");
    std::env::set_current_dir(&temp_dir)
        .expect("Failed to change dir");

    let result = check_index_exists();

    std::env::set_current_dir(&original_dir)
        .expect("Failed to restore dir");

    let _ = fs::remove_dir_all(&temp_dir);

    assert!(result);
}

/// analyze_codebase returns valid analysis
#[test]
fn test_analyze_codebase() {
    let temp_dir =
        std::env::temp_dir().join("test-analyze");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir)
        .expect("Failed to create temp dir");

    let src_file = temp_dir.join("main.rs");
    fs::write(&src_file, "fn main() {}")
        .expect("Failed to write file");

    let original_dir = std::env::current_dir()
        .expect("Failed to get current dir");
    std::env::set_current_dir(&temp_dir)
        .expect("Failed to change dir");

    let analysis = analyze_codebase();

    std::env::set_current_dir(&original_dir)
        .expect("Failed to restore dir");

    let _ = fs::remove_dir_all(&temp_dir);

    assert!(analysis.primary_language.is_some());
    assert_eq!(analysis.total_source_files, 1);
}

/// detect_codebase_changes returns None with no index
#[test]
fn test_detect_codebase_changes_no_index() {
    let temp_dir = std::env::temp_dir()
        .join("test-changes-no-index-v2");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir)
        .expect("Failed to create temp dir");

    let original_dir = std::env::current_dir()
        .expect("Failed to get current dir");

    std::env::set_current_dir(&temp_dir)
        .expect("Failed to change dir");

    let result = detect_codebase_changes();

    std::env::set_current_dir(&original_dir)
        .expect("Failed to restore dir");

    let _ = fs::remove_dir_all(&temp_dir);

    assert!(result.is_none());
}
