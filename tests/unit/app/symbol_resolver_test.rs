//! Tests for app::handlers::symbol_resolver

use rustean::app::handlers::symbol_resolver;
use rustean::message::MessageSegment;

/// resolve_symbols skips non-SymbolReference segments
#[test]
fn resolve_skips_text_segments() {
    // Arrange
    let mut segments = vec![
        MessageSegment::Text("hello".to_string()),
    ];

    // Act
    symbol_resolver::resolve_symbols(
        &mut segments, &[],
    );

    // Assert
    assert!(matches!(
        &segments[0],
        MessageSegment::Text(t) if t == "hello"
    ));
}

/// resolve_symbols preserves existing source_code
#[test]
fn resolve_skips_existing_source_code() {
    // Arrange
    let existing = "fn foo() {}".to_string();
    let mut segments = vec![
        MessageSegment::SymbolReference {
            full_path: "test.rs".to_string(),
            display_name: "test.rs(foo)".into(),
            symbol_path: "foo".to_string(),
            source_code: Some(existing.clone()),
        },
    ];

    // Act
    symbol_resolver::resolve_symbols(
        &mut segments, &[],
    );

    // Assert
    let MessageSegment::SymbolReference {
        source_code, ..
    } = &segments[0]
    else {
        panic!("expected SymbolReference");
    };
    assert_eq!(source_code.as_deref(), Some("fn foo() {}"));
}

/// resolve_symbols sets None for nonexistent file
#[test]
fn resolve_nonexistent_file_returns_none() {
    // Arrange
    let mut segments = vec![
        MessageSegment::SymbolReference {
            full_path: "/nonexistent.rs".into(),
            display_name: "nofile.rs(foo)".into(),
            symbol_path: "foo".to_string(),
            source_code: None,
        },
    ];

    // Act
    symbol_resolver::resolve_symbols(
        &mut segments, &[],
    );

    // Assert
    let MessageSegment::SymbolReference {
        source_code, ..
    } = &segments[0]
    else {
        panic!("expected SymbolReference");
    };
    assert!(source_code.is_none());
}

/// resolve_symbols with empty slice is a noop
#[test]
fn resolve_empty_segments_is_noop() {
    // Arrange
    let mut segments: Vec<MessageSegment> = vec![];

    // Act
    symbol_resolver::resolve_symbols(
        &mut segments, &[],
    );

    // Assert
    assert!(segments.is_empty());
}

/// resolve_symbols skips FileReference and FolderReference
#[test]
fn resolve_skips_file_and_folder_refs() {
    // Arrange
    let mut segments = vec![
        MessageSegment::FileReference {
            full_path: "/src/main.rs".into(),
            display_name: "main.rs".into(),
        },
        MessageSegment::FolderReference {
            full_path: "/src".into(),
            display_name: "src".into(),
        },
    ];

    // Act
    symbol_resolver::resolve_symbols(
        &mut segments, &[],
    );

    // Assert — segments remain unchanged
    assert!(matches!(
        &segments[0],
        MessageSegment::FileReference { .. }
    ));
    assert!(matches!(
        &segments[1],
        MessageSegment::FolderReference { .. }
    ));
}
