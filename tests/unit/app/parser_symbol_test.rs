//! Tests for parser with SymbolSelector integration

use rustean::app::parser::parse_input_to_message;
use rustean::domain::{
    FileName, FilePath, InputSpan, SymbolSelector,
};
use rustean::message::MessageSegment;

/// Helper: create a SymbolSelector
fn make_selector(
    start: usize,
    end: usize,
    symbol_path: &str,
) -> SymbolSelector {
    let span = InputSpan { start, end };
    SymbolSelector::new(
        span,
        FilePath::from_string("src/app.rs"),
        FileName::new("app.rs".to_string()),
        symbol_path.to_string(),
    )
}

// -----------------------------------------------------------
// Symbol ref parsing tests
// -----------------------------------------------------------

/// Symbol ref creates a SymbolReference segment
#[test]
fn test_parse_symbol_ref_creates_segment() {
    // Input: "check app.rs(App::run) now"
    //         01234567890123456789012345
    let sel = make_selector(6, 22, "App::run");
    let result = parse_input_to_message(
        "check app.rs(App::run) now".to_string(),
        &[],
        &[sel],
    );

    assert!(result.is_some());
    let message = result.unwrap();
    // "check ", symbol_ref, " now"
    assert_eq!(message.segments.len(), 3);

    let has_symbol = message.segments.iter().any(|seg| {
        matches!(
            seg,
            MessageSegment::SymbolReference { .. }
        )
    });
    assert!(has_symbol);
}

/// Empty input with symbol refs returns None
#[test]
fn test_parse_empty_with_symbol_ref() {
    let result = parse_input_to_message(
        "".to_string(),
        &[],
        &[],
    );
    assert!(result.is_none());
}

/// Text-only input with empty sym refs works
#[test]
fn test_parse_text_no_symbol_refs() {
    let result = parse_input_to_message(
        "hello".to_string(),
        &[],
        &[],
    );
    assert!(result.is_some());
    let message = result.unwrap();
    assert_eq!(message.segments.len(), 1);
}

/// Symbol ref display_name uses compact format
#[test]
fn test_symbol_ref_display_name() {
    let sel = make_selector(0, 16, "App::run");
    let result = parse_input_to_message(
        "app.rs(App::run)".to_string(),
        &[],
        &[sel],
    );

    let message = result.unwrap();
    let seg = &message.segments[0];
    let display = seg.display_text();
    assert!(display.contains("App::run"));
}
