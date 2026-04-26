//! Unit tests for startup::checks

use std::fs;

use rustean::indexer::CodebaseAnalyzer;
use rustean::startup::{
    check_index_exists, detect_codebase_changes,
};

/// Returns false when no index directory exists
#[test]
fn check_index_exists_no_index() {
    let temp_dir =
        std::env::temp_dir().join("test-no-index");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir)
        .expect("create temp dir");

    let original_dir = std::env::current_dir()
        .expect("get current dir");
    std::env::set_current_dir(&temp_dir)
        .expect("change dir");

    let result = check_index_exists();

    std::env::set_current_dir(&original_dir)
        .expect("restore dir");
    let _ = fs::remove_dir_all(&temp_dir);

    assert!(!result);
}

/// Returns true when index state file exists
#[test]
fn check_index_exists_with_index() {
    let temp_dir = std::env::temp_dir()
        .join("test-with-index-v2");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir)
        .expect("create temp dir");
    let index_dir =
        rustean::domain::data_paths_dirs::index_dir(
            &temp_dir,
        );
    fs::create_dir_all(&index_dir)
        .expect("create index dir");
    fs::write(
        index_dir.join("state.json"),
        r#"{"version": "1.0", "files": []}"#,
    )
    .expect("write state file");

    let original_dir = std::env::current_dir()
        .expect("get current dir");
    std::env::set_current_dir(&temp_dir)
        .expect("change dir");

    let result = check_index_exists();

    std::env::set_current_dir(&original_dir)
        .expect("restore dir");
    let _ = fs::remove_dir_all(&temp_dir);

    assert!(result);
}

/// Analyzer detects single Rust source file
#[test]
fn analyze_codebase_detects_rust_file() {
    let temp_dir =
        std::env::temp_dir().join("test-analyze");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir)
        .expect("create temp dir");
    fs::write(
        temp_dir.join("main.rs"),
        "fn main() {}",
    )
    .expect("write file");

    let analyzer = CodebaseAnalyzer::new();
    let analysis = analyzer.analyze(
        temp_dir.to_str().unwrap(),
    );

    let _ = fs::remove_dir_all(&temp_dir);

    let lang = analysis.primary_language
        .expect("should detect a language");
    assert_eq!(lang.display_name(), "Rust");
    assert_eq!(analysis.total_source_files, 1);
}

/// Returns None when no index state exists
#[test]
fn detect_codebase_changes_no_index() {
    let temp_dir = std::env::temp_dir()
        .join("test-changes-no-index-v2");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir)
        .expect("create temp dir");

    let original_dir = std::env::current_dir()
        .expect("get current dir");
    std::env::set_current_dir(&temp_dir)
        .expect("change dir");

    let result = detect_codebase_changes();

    std::env::set_current_dir(&original_dir)
        .expect("restore dir");
    let _ = fs::remove_dir_all(&temp_dir);

    assert!(result.is_none());
}
