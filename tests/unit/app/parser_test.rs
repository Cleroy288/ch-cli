//! Unit tests for app::parser — migrated from inline tests

use ch_cli::app::parser::parse_input_to_message;
use ch_cli::domain::{FileName, FilePath, FileReference};

#[test]
fn test_parse_empty_input() {
    let result =
        parse_input_to_message(String::new(), &[]);
    assert!(result.is_none());
}

#[test]
fn test_parse_whitespace_only() {
    let result = parse_input_to_message(
        "   \n  ".to_string(),
        &[],
    );
    assert!(result.is_none());
}

#[test]
fn test_parse_text_only() {
    let result = parse_input_to_message(
        "Hello world".to_string(),
        &[],
    );
    assert!(result.is_some());
    let message = result.unwrap();
    assert_eq!(message.segments.len(), 1);
}

#[test]
fn test_parse_with_file_reference() {
    let file_ref = FileReference::new(
        6,
        14,
        FilePath::from_string("./src/main.rs"),
        FileName::new("main.rs".to_string()),
        false,
    );
    let result = parse_input_to_message(
        "Check main.rs file".to_string(),
        &[file_ref],
    );
    assert!(result.is_some());
    let message = result.unwrap();
    // "Check ", file_ref, " file"
    assert_eq!(message.segments.len(), 3);
}
