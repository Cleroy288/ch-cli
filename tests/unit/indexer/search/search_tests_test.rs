//! Tests for the search module.

use std::path::PathBuf;

use rustean::indexer::search::SearchIndex;
use rustean::indexer::symbols::{
    ByteSpan, CodeLocation, Symbol, SymbolKind,
};

/// Helper to create a test symbol with default path
fn create_test_symbol(name: &str, kind: SymbolKind) -> Symbol {
    Symbol::new(
        name.to_string(),
        kind,
        CodeLocation::new(PathBuf::from("test.rs"), 1, 0, ByteSpan { offset: 0, length: 10 }),
    )
}

/// Helper to create a test symbol at a specific file path
fn create_test_symbol_with_path(name: &str, kind: SymbolKind, path: &str) -> Symbol {
    Symbol::new(
        name.to_string(),
        kind,
        CodeLocation::new(PathBuf::from(path), 1, 0, ByteSpan { offset: 0, length: 10 }),
    )
}

#[test]
fn test_index_and_search() {
    let index = SearchIndex::in_memory().unwrap(); // create in-memory index

    let symbols = vec![
        create_test_symbol("handle_events", SymbolKind::Function),
        create_test_symbol("process_input", SymbolKind::Function),
        create_test_symbol("EventHandler", SymbolKind::Struct),
    ];

    index.index_symbols(&symbols).unwrap();

    // Search for "handle"
    let results = index.search("handle", 10).unwrap();
    assert!(!results.is_empty());
    assert_eq!(results[0].symbol.name, "handle_events");
}

#[test]
fn test_fuzzy_search() {
    let index = SearchIndex::in_memory().unwrap(); // create in-memory index

    let symbols = vec![create_test_symbol("function_name", SymbolKind::Function)];
    index.index_symbols(&symbols).unwrap();

    // Search with typo
    let results = index.fuzzy_search("functon", 2, 10).unwrap();
    assert!(!results.is_empty());
}

#[test]
fn test_search_by_kind() {
    let index = SearchIndex::in_memory().unwrap(); // create in-memory index

    let symbols = vec![
        create_test_symbol("my_func", SymbolKind::Function),
        create_test_symbol("MyStruct", SymbolKind::Struct),
        create_test_symbol("another_func", SymbolKind::Function),
    ];
    index.index_symbols(&symbols).unwrap();

    let results = index.search_by_kind(SymbolKind::Function, 10).unwrap();
    assert_eq!(results.len(), 2);
}

/// document_type field is indexed and stored for all paths
#[test]
fn test_document_type_field_indexed() {
    let index = SearchIndex::in_memory().unwrap(); // create in-memory index

    // Create symbols from different document types
    let symbols = vec![
        create_test_symbol_with_path("source_func", SymbolKind::Function, "src/main.rs"),
        create_test_symbol_with_path("doc_func", SymbolKind::Function, "doc/api.md"),
        create_test_symbol_with_path("notes_func", SymbolKind::Function, "notes/impl.md"),
        create_test_symbol_with_path("test_func", SymbolKind::Function, "tests/unit.rs"),
    ];

    index.index_symbols(&symbols).unwrap();

    // Verify all symbols are indexed
    let num_docs = index.num_docs().unwrap();
    assert_eq!(num_docs, 4);

    // Search should return all matching symbols
    let results = index.search("func", 10).unwrap();
    assert_eq!(results.len(), 4);
}

/// search_with_boost ranks source code above docs/notes
#[test]
fn test_search_with_boost_prefers_source_code() {
    let index = SearchIndex::in_memory().unwrap(); // create in-memory index

    // Create symbols with same name but different document types
    let symbols = vec![
        create_test_symbol_with_path("handle_data", SymbolKind::Function, "notes/impl.md"),
        create_test_symbol_with_path("handle_data", SymbolKind::Function, "src/handler.rs"),
        create_test_symbol_with_path("handle_data", SymbolKind::Function, "doc/api.md"),
    ];

    index.index_symbols(&symbols).unwrap();

    // Boosted search should rank source code higher
    let results = index.search_with_boost("handle_data", 10).unwrap();
    assert!(!results.is_empty());

    // First result should be from source code (src/handler.rs)
    let first_path = results[0].symbol.location.file.to_string_lossy();
    assert!(
        first_path.contains("src/"),
        "Expected source code file first, got: {}",
        first_path
    );
}
