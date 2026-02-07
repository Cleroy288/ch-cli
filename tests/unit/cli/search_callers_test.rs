//! Unit tests for cli::commands::search_callers
//! — migrated from inline tests

use ch_cli::cli::commands::search_callers::handle_caller_query;
use ch_cli::indexer::semantic::SemanticGraph;
use ch_cli::retrieval::query::{
    CallerDirection, CallerQuery,
};

#[test]
fn caller_query_empty_graph() {
    let graph = SemanticGraph::new();
    let query = CallerQuery {
        symbol_name: "test_symbol".into(),
        direction: CallerDirection::Callers,
    };
    let result = handle_caller_query(&query, &graph);
    assert!(result.is_ok());
}

#[test]
fn callee_query_empty_graph() {
    let graph = SemanticGraph::new();
    let query = CallerQuery {
        symbol_name: "test_symbol".into(),
        direction: CallerDirection::Callees,
    };
    let result = handle_caller_query(&query, &graph);
    assert!(result.is_ok());
}
