//! Unit tests for domain::symbol_ref

use rustean::domain::symbol_ref::{
    build_symbol_path, extract_leaf_name,
    SymbolSelector,
};
use rustean::domain::{
    FileName, FilePath, InputSpan,
};

// -----------------------------------------------------------
// SymbolSelector::display_text tests
// -----------------------------------------------------------

/// Display text shows compact format
#[test]
fn test_display_text_simple() {
    let span = InputSpan { start: 0, end: 20 };
    let selector = SymbolSelector::new(
        span,
        FilePath::from_string("src/app.rs"),
        FileName::new("app.rs".to_string()),
        "App::run".to_string(),
    );

    assert_eq!(
        selector.display_text(),
        "app.rs(App::run)"
    );
}

/// Display text for top-level symbol
#[test]
fn test_display_text_top_level() {
    let span = InputSpan { start: 0, end: 10 };
    let selector = SymbolSelector::new(
        span,
        FilePath::from_string("src/main.rs"),
        FileName::new("main.rs".to_string()),
        "main".to_string(),
    );

    assert_eq!(
        selector.display_text(),
        "main.rs(main)"
    );
}

// -----------------------------------------------------------
// build_symbol_path tests
// -----------------------------------------------------------

/// Build path with no parents
#[test]
fn test_build_symbol_path_no_parents() {
    let result = build_symbol_path(&[], "main");
    assert_eq!(result, "main");
}

/// Build path with one parent
#[test]
fn test_build_symbol_path_one_parent() {
    let parents = vec!["App".to_string()];
    let result = build_symbol_path(&parents, "run");
    assert_eq!(result, "App::run");
}

/// Build path with multiple parents
#[test]
fn test_build_symbol_path_multiple_parents() {
    let parents = vec![
        "Module".to_string(),
        "Struct".to_string(),
    ];
    let result =
        build_symbol_path(&parents, "method");
    assert_eq!(result, "Module::Struct::method");
}

// -----------------------------------------------------------
// SymbolSelector fields tests
// -----------------------------------------------------------

/// Selector stores span positions correctly
#[test]
fn test_selector_stores_positions() {
    let span = InputSpan { start: 5, end: 25 };
    let selector = SymbolSelector::new(
        span,
        FilePath::from_string("test.rs"),
        FileName::new("test.rs".to_string()),
        "Foo".to_string(),
    );

    assert_eq!(selector.start, 5);
    assert_eq!(selector.end, 25);
    assert_eq!(selector.symbol_path, "Foo");
}

// -----------------------------------------------------------
// extract_leaf_name tests
// -----------------------------------------------------------

/// Extracts leaf from nested path
#[test]
fn test_extract_leaf_name_nested() {
    assert_eq!(
        extract_leaf_name("Calculator::add"),
        "add",
    );
}

/// Returns same string when no :: present
#[test]
fn test_extract_leaf_name_simple() {
    assert_eq!(extract_leaf_name("main"), "main");
}

/// Handles deeply nested paths
#[test]
fn test_extract_leaf_name_deep() {
    assert_eq!(
        extract_leaf_name("A::B::C::method"),
        "method",
    );
}

/// Handles empty string
#[test]
fn test_extract_leaf_name_empty() {
    assert_eq!(extract_leaf_name(""), "");
}
