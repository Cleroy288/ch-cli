//! Unit tests for SymbolBrowser

use std::path::PathBuf;

use rustean::indexer::symbols::{
    CodeLocation, ByteSpan, Symbol, SymbolKind,
};
use rustean::picker::SymbolBrowser;

/// Helper: build a minimal Symbol with name, kind, parent
fn make_symbol(
    name: &str,
    kind: SymbolKind,
    parent: Option<&str>,
) -> Symbol {
    let mut sym = Symbol::new(
        name.to_string(),
        kind,
        CodeLocation::new(
            PathBuf::from("test.rs"),
            1,
            1,
            ByteSpan::ZERO,
        ),
    );
    sym.parent = parent.map(|s| s.to_string());
    sym
}

/// Build a test set of symbols
fn test_symbols() -> Vec<Symbol> {
    vec![
        make_symbol("App", SymbolKind::Struct, None),
        make_symbol("run", SymbolKind::Method, Some("App")),
        make_symbol("stop", SymbolKind::Method, Some("App")),
        make_symbol("Color", SymbolKind::Enum, None),
        make_symbol("Red", SymbolKind::EnumVariant, Some("Color")),
        make_symbol("main", SymbolKind::Function, None),
    ]
}

// -----------------------------------------------------------
// current_items tests
// -----------------------------------------------------------

/// Top-level items returns symbols without parent
#[test]
fn test_current_items_top_level() {
    let browser = SymbolBrowser::new(
        PathBuf::from("test.rs"),
        test_symbols(),
    );

    let items = browser.current_items("");
    let names: Vec<&str> =
        items.iter().map(|sym| sym.name.as_str()).collect();

    assert_eq!(names, vec!["App", "Color", "main"]);
}

/// Filtering by query narrows results
#[test]
fn test_current_items_with_query() {
    let browser = SymbolBrowser::new(
        PathBuf::from("test.rs"),
        test_symbols(),
    );

    let items = browser.current_items("app");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "App");
}

/// Empty query returns all at current level
#[test]
fn test_current_items_empty_query() {
    let browser = SymbolBrowser::new(
        PathBuf::from("test.rs"),
        test_symbols(),
    );

    let items = browser.current_items("");
    assert_eq!(items.len(), 3);
}

// -----------------------------------------------------------
// drill_into tests
// -----------------------------------------------------------

/// Drilling into a struct shows its children
#[test]
fn test_drill_into_shows_children() {
    let mut browser = SymbolBrowser::new(
        PathBuf::from("test.rs"),
        test_symbols(),
    );

    browser.drill_into("App".to_string());

    let items = browser.current_items("");
    let names: Vec<&str> =
        items.iter().map(|sym| sym.name.as_str()).collect();

    assert_eq!(names, vec!["run", "stop"]);
}

/// Drilling into enum shows variants
#[test]
fn test_drill_into_enum() {
    let mut browser = SymbolBrowser::new(
        PathBuf::from("test.rs"),
        test_symbols(),
    );

    browser.drill_into("Color".to_string());

    let items = browser.current_items("");
    assert_eq!(items.len(), 1);
    assert_eq!(items[0].name, "Red");
}

// -----------------------------------------------------------
// go_up tests
// -----------------------------------------------------------

/// go_up returns true when navigating back
#[test]
fn test_go_up_returns_true() {
    let mut browser = SymbolBrowser::new(
        PathBuf::from("test.rs"),
        test_symbols(),
    );
    browser.drill_into("App".to_string());

    let result = browser.go_up();

    assert!(result);
    assert!(browser.current_parent().is_none());
}

/// go_up returns false at top level
#[test]
fn test_go_up_at_top_returns_false() {
    let mut browser = SymbolBrowser::new(
        PathBuf::from("test.rs"),
        test_symbols(),
    );

    let result = browser.go_up();

    assert!(!result);
}

// -----------------------------------------------------------
// is_container tests
// -----------------------------------------------------------

/// Struct, Enum, Trait, Impl are containers
#[test]
fn test_is_container_true() {
    assert!(SymbolBrowser::is_container(SymbolKind::Struct));
    assert!(SymbolBrowser::is_container(SymbolKind::Enum));
    assert!(SymbolBrowser::is_container(SymbolKind::Trait));
    assert!(SymbolBrowser::is_container(SymbolKind::Impl));
}

/// Function, Method, etc. are not containers
#[test]
fn test_is_container_false() {
    assert!(!SymbolBrowser::is_container(SymbolKind::Function));
    assert!(!SymbolBrowser::is_container(SymbolKind::Method));
    assert!(!SymbolBrowser::is_container(SymbolKind::Constant));
}

// -----------------------------------------------------------
// parent_stack tests
// -----------------------------------------------------------

/// parent_stack tracks drill history
#[test]
fn test_parent_stack_tracks_history() {
    let mut browser = SymbolBrowser::new(
        PathBuf::from("test.rs"),
        test_symbols(),
    );

    browser.drill_into("App".to_string());
    assert_eq!(browser.parent_stack(), &["App"]);

    browser.go_up();
    assert!(browser.parent_stack().is_empty());
}

// -----------------------------------------------------------
// doc_names tests
// -----------------------------------------------------------

/// has_doc returns false when no doc names set
#[test]
fn test_has_doc_empty_by_default() {
    let browser = SymbolBrowser::new(
        PathBuf::from("test.rs"),
        test_symbols(),
    );

    assert!(!browser.has_doc("App"));
    assert!(!browser.has_doc("main"));
}

/// has_doc returns true for documented symbols
#[test]
fn test_has_doc_after_set_doc_names() {
    use std::collections::HashSet;

    let mut browser = SymbolBrowser::new(
        PathBuf::from("test.rs"),
        test_symbols(),
    );
    let mut names = HashSet::new();
    names.insert("App".to_string());
    names.insert("main".to_string());
    browser.set_doc_names(names);

    assert!(browser.has_doc("App"));
    assert!(browser.has_doc("main"));
    assert!(!browser.has_doc("run"));
}
