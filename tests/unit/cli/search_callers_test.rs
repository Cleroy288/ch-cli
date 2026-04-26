//! Tests for search_callers on empty graph.
//!
//! handle_caller_query prints output to stdout,
//! so we verify it completes without error.

use rustean::cli::commands::search_callers
    ::handle_caller_query;
use rustean::indexer::semantic::SemanticGraph;
use rustean::service::search::caller::{
    CallerDirection, CallerQuery,
};

/// Caller query on empty graph succeeds
#[test]
fn caller_query_empty_graph_succeeds() {
    // Arrange
    let graph = SemanticGraph::new();
    let query = CallerQuery {
        symbol_name: "test_symbol".into(),
        direction: CallerDirection::Callers,
    };

    // Act + Assert
    handle_caller_query(&query, &graph).unwrap();
}

/// Callee query on empty graph succeeds
#[test]
fn callee_query_empty_graph_succeeds() {
    // Arrange
    let graph = SemanticGraph::new();
    let query = CallerQuery {
        symbol_name: "test_symbol".into(),
        direction: CallerDirection::Callees,
    };

    // Act + Assert
    handle_caller_query(&query, &graph).unwrap();
}
