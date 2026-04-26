//! Unit tests for domain::symbol_ref

use rustean::domain::symbol_ref::{
    build_symbol_path, extract_leaf_name,
    SymbolSelector,
};
use rustean::domain::{
    FileName, FilePath, InputSpan,
};

/// Display text shows compact format
#[test]
fn display_text_nested_symbol() {
    let span = InputSpan { start: 0, end: 20 };
    let path: FilePath = "src/app.rs".into();
    let selector = SymbolSelector::new(
        span,
        path,
        FileName::from("app.rs".to_string()),
        "App::run".to_string(),
    );

    assert_eq!(
        selector.display_text(),
        "app.rs(App::run)",
    );
}

/// Display text for top-level symbol
#[test]
fn display_text_top_level_symbol() {
    let span = InputSpan { start: 0, end: 10 };
    let path: FilePath = "src/main.rs".into();
    let selector = SymbolSelector::new(
        span,
        path,
        FileName::from("main.rs".to_string()),
        "main".to_string(),
    );

    assert_eq!(
        selector.display_text(),
        "main.rs(main)",
    );
}

/// build_symbol_path with varying parent depth
#[test]
fn build_symbol_path_cases() {
    let cases: &[(&[&str], &str, &str)] = &[
        (&[], "main", "main"),
        (&["App"], "run", "App::run"),
        (
            &["Module", "Struct"],
            "method",
            "Module::Struct::method",
        ),
    ];
    for (parents, leaf, expected) in cases {
        let owned: Vec<String> = parents
            .iter()
            .map(|s| s.to_string())
            .collect();
        let result =
            build_symbol_path(&owned, leaf);
        assert_eq!(
            result, *expected,
            "build_symbol_path({parents:?}, {leaf})",
        );
    }
}

/// Selector stores span positions correctly
#[test]
fn selector_stores_positions() {
    let span = InputSpan { start: 5, end: 25 };
    let path: FilePath = "test.rs".into();
    let selector = SymbolSelector::new(
        span,
        path,
        FileName::from("test.rs".to_string()),
        "Foo".to_string(),
    );

    assert_eq!(selector.start, 5);
    assert_eq!(selector.end, 25);
    assert_eq!(selector.symbol_path, "Foo");
}

/// extract_leaf_name extracts last :: segment
#[test]
fn extract_leaf_name_cases() {
    let cases = [
        ("Calculator::add", "add"),
        ("main", "main"),
        ("A::B::C::method", "method"),
        ("", ""),
    ];
    for (input, expected) in cases {
        assert_eq!(
            extract_leaf_name(input), expected,
            "extract_leaf_name({input:?})",
        );
    }
}
