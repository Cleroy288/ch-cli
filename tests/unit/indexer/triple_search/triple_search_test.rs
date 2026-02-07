//! Triple Search Tests
//!
//! Unit tests for triple search index functionality.

use std::path::PathBuf;

use ch_cli::indexer::symbols::{CodeLocation, Symbol, SymbolKind};
use ch_cli::indexer::triple_search::TripleSearchIndex;

/// Create a test symbol at given path
fn create_test_symbol(name: &str, path: &str) -> Symbol {
    Symbol::new(
        name.to_string(),
        SymbolKind::Function,
        CodeLocation::new(PathBuf::from(path), 1, 0, 0, 10),
    )
}

/// Test TripleSearchIndex routes symbols correctly
#[test]
fn test_triple_index_routes_symbols() {
    let index = TripleSearchIndex::in_memory().unwrap();

    // Use paths with leading / to match DocumentType patterns
    let symbols = vec![
        create_test_symbol("code_func", "/project/src/main.rs"),
        create_test_symbol("test_func", "/project/tests/test.rs"),
        create_test_symbol("doc_section", "/project/doc/guide.md"),
        create_test_symbol("note_section", "/project/notes/impl.md"),
        create_test_symbol("bench_section", "/project/notes/benchmarks/perf.md"),
    ];

    let stats = index.index_symbols(&symbols).unwrap();

    // Code: src/main.rs + tests/test.rs = 2
    assert_eq!(stats.code_count, 2);
    // Doc: doc/guide.md = 1
    assert_eq!(stats.doc_count, 1);
    // Notes: notes/impl.md + notes/benchmarks/perf.md = 2
    assert_eq!(stats.notes_count, 2);
    // Total: 5
    assert_eq!(stats.total(), 5);
}

/// Test TripleSearchIndex searches code index only
#[test]
fn test_triple_index_search_code() {
    let index = TripleSearchIndex::in_memory().unwrap();

    let symbols = vec![
        create_test_symbol("handle_request", "/project/src/handler.rs"),
        create_test_symbol("handle_docs", "/project/doc/api.md"),
    ];

    index.index_symbols(&symbols).unwrap();

    // Search code index - should find handle_request
    let results = index.search_code("handle", 10).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].symbol.name, "handle_request");
}

/// Test TripleSearchIndex searches doc index only
#[test]
fn test_triple_index_search_docs() {
    let index = TripleSearchIndex::in_memory().unwrap();

    let symbols = vec![
        create_test_symbol("handle_request", "/project/src/handler.rs"),
        create_test_symbol("handle_docs", "/project/doc/api.md"),
    ];

    index.index_symbols(&symbols).unwrap();

    // Search doc index - should find handle_docs
    let results = index.search_docs("handle", 10).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].symbol.name, "handle_docs");
}

/// Test TripleSearchIndex searches notes index only
#[test]
fn test_triple_index_search_notes() {
    let index = TripleSearchIndex::in_memory().unwrap();

    let symbols = vec![
        create_test_symbol("handle_request", "/project/src/handler.rs"),
        create_test_symbol("handle_notes", "/project/notes/impl.md"),
    ];

    index.index_symbols(&symbols).unwrap();

    // Search notes index - should find handle_notes
    let results = index.search_notes("handle", 10).unwrap();
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].symbol.name, "handle_notes");
}

/// Test TripleSearchIndex parallel search returns all results
#[test]
fn test_triple_index_parallel_search() {
    let index = TripleSearchIndex::in_memory().unwrap();

    let symbols = vec![
        create_test_symbol("handle_request", "/project/src/handler.rs"),
        create_test_symbol("handle_docs", "/project/doc/api.md"),
        create_test_symbol("handle_notes", "/project/notes/impl.md"),
    ];

    index.index_symbols(&symbols).unwrap();

    // Parallel search all indexes
    let results = index.search_parallel("handle", 10, 10, 10).unwrap();

    assert_eq!(results.code_results.len(), 1);
    assert_eq!(results.doc_results.len(), 1);
    assert_eq!(results.notes_results.len(), 1);

    assert_eq!(results.code_results[0].symbol.name, "handle_request");
    assert_eq!(results.doc_results[0].symbol.name, "handle_docs");
    assert_eq!(results.notes_results[0].symbol.name, "handle_notes");
}

// ==================== Extra Tests ====================

/// Test TripleSearchIndex with empty symbols
#[test]
fn test_triple_index_empty() {
    let index = TripleSearchIndex::in_memory().unwrap();

    let stats = index.index_symbols(&[]).unwrap();

    assert_eq!(stats.code_count, 0);
    assert_eq!(stats.doc_count, 0);
    assert_eq!(stats.notes_count, 0);
    assert_eq!(stats.total(), 0);
}

/// Test TripleSearchIndex respects limits
#[test]
fn test_triple_index_respects_limits() {
    let index = TripleSearchIndex::in_memory().unwrap();

    // Use names that share a common word for Tantivy tokenization
    let symbols = vec![
        create_test_symbol("handle_alpha", "/project/src/a.rs"),
        create_test_symbol("handle_beta", "/project/src/b.rs"),
        create_test_symbol("handle_gamma", "/project/src/c.rs"),
    ];

    index.index_symbols(&symbols).unwrap();

    // Search with limit of 1
    let results = index.search_code("handle", 1).unwrap();
    assert_eq!(results.len(), 1);

    // Search with limit of 2
    let results = index.search_code("handle", 2).unwrap();
    assert_eq!(results.len(), 2);
}
