//! Tests for caller context embedding enrichment.

use std::path::PathBuf;

use rustean::indexer::{
	ByteSpan, CodeLocation, ReferenceContext,
	SemanticGraph, SymbolReference,
};
use rustean::retrieval::hybrid::embedding_callers::{
	caller_context, extract_caller_names,
};

/// Build a Call reference at the given file path
fn make_call_ref(
	name: &str,
	file: &str,
) -> SymbolReference {
	SymbolReference {
		name: name.to_string(),
		location: CodeLocation::new(
			PathBuf::from(file),
			10, 1, ByteSpan::ZERO,
		),
		context: ReferenceContext::Call,
	}
}

/// Build a reference with a specific context
fn make_ref_with_context(
	name: &str,
	file: &str,
	ctx: ReferenceContext,
) -> SymbolReference {
	SymbolReference {
		name: name.to_string(),
		location: CodeLocation::new(
			PathBuf::from(file),
			5, 1, ByteSpan::ZERO,
		),
		context: ctx,
	}
}

#[test]
fn caller_context_returns_none_when_no_callers() {
	// Arrange — empty graph, no references at all
	let graph = SemanticGraph::new();

	// Act
	let result = caller_context("foo", &graph);

	// Assert
	assert!(result.is_none());
}

#[test]
fn caller_context_formats_caller_modules() {
	// Arrange — two Call refs from different files
	let mut graph = SemanticGraph::new();
	graph.add_reference(make_call_ref(
		"process",
		"src/service/handler.rs",
	));
	graph.add_reference(make_call_ref(
		"process",
		"src/retrieval/pipeline.rs",
	));

	// Act
	let result = caller_context("process", &graph);

	// Assert — both modules present
	let text = result.expect("should have callers");
	assert!(text.starts_with("called from "));
	assert!(text.contains("retrieval pipeline"));
	assert!(text.contains("service handler"));
}

#[test]
fn extract_caller_names_limits_to_max() {
	// Arrange — 5 Call refs from different files
	let mut graph = SemanticGraph::new();
	let files = [
		"src/a/alpha.rs",
		"src/b/bravo.rs",
		"src/c/charlie.rs",
		"src/d/delta.rs",
		"src/e/echo.rs",
	];
	for file in &files {
		graph.add_reference(
			make_call_ref("target", file),
		);
	}

	// Act
	let names = extract_caller_names("target", &graph);

	// Assert — capped at MAX_CALLERS (3)
	assert_eq!(names.len(), 3);
}

#[test]
fn caller_context_filters_non_call_refs() {
	// Arrange — mix of Call, Type, and Import refs
	let mut graph = SemanticGraph::new();
	graph.add_reference(make_call_ref(
		"MyStruct",
		"src/service/caller.rs",
	));
	graph.add_reference(make_ref_with_context(
		"MyStruct",
		"src/domain/types.rs",
		ReferenceContext::Type,
	));
	graph.add_reference(make_ref_with_context(
		"MyStruct",
		"src/handler/api.rs",
		ReferenceContext::Import,
	));

	// Act
	let names =
		extract_caller_names("MyStruct", &graph);

	// Assert — only the Call ref is included
	assert_eq!(names.len(), 1);
	assert!(names[0].contains("service caller"));
}
