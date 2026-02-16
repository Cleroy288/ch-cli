//! Unit tests for app::parser

use rustean::app::parser::parse_input_to_message;
use rustean::domain::{
    FileName, FilePath, FileReference, InputSpan,
};

#[test]
fn test_parse_empty_input() {
    let result = parse_input_to_message(
        String::new(),
        &[],
        &[],
    );
    assert!(result.is_none());
}

#[test]
fn test_parse_whitespace_only() {
    let result = parse_input_to_message(
        "   \n  ".to_string(),
        &[],
        &[],
    );
    assert!(result.is_none());
}

#[test]
fn test_parse_text_only() {
    let result = parse_input_to_message(
        "Hello world".to_string(),
        &[],
        &[],
    );
    assert!(result.is_some());
    let message = result.unwrap();
    assert_eq!(message.segments.len(), 1);
}

#[test]
fn test_parse_with_file_reference() {
    let span = InputSpan { start: 6, end: 14 };
    let file_ref = FileReference::new(
        span,
        FilePath::from_string("./src/main.rs"),
        FileName::new("main.rs".to_string()),
        false,
    );
    let result = parse_input_to_message(
        "Check main.rs file".to_string(),
        &[file_ref],
        &[],
    );
    assert!(result.is_some());
    let message = result.unwrap();
    // "Check ", file_ref, " file"
    assert_eq!(message.segments.len(), 3);
}
