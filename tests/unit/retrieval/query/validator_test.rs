use rustean::indexer::{
	ByteSpan, CodeLocation, ReferenceContext,
	SemanticGraph, Symbol, SymbolKind,
	SymbolReference, Visibility,
};
use rustean::retrieval::query::SymbolValidator;
use std::path::PathBuf;

fn create_test_graph() -> SemanticGraph {
	let mut graph = SemanticGraph::new();

	let auth = Symbol::new(
		"AuthService".to_string(),
		SymbolKind::Struct,
		CodeLocation::new(
			PathBuf::from("src/auth.rs"),
			10, 1,
			ByteSpan { offset: 0, length: 11 },
		),
	)
	.with_visibility(Visibility::Public);
	graph.add_definition(auth);

	for i in 0..10 {
		graph.add_reference(SymbolReference {
			name: "AuthService".to_string(),
			location: CodeLocation::new(
				PathBuf::from("src/main.rs"),
				20 + i, 1,
				ByteSpan { offset: 0, length: 11 },
			),
			context: ReferenceContext::Type,
		});
	}

	let helper = Symbol::new(
		"helper_fn".to_string(),
		SymbolKind::Function,
		CodeLocation::new(
			PathBuf::from("src/utils.rs"),
			5, 1,
			ByteSpan { offset: 0, length: 9 },
		),
	);
	graph.add_definition(helper);

	graph
}

#[test]
fn test_validate_existing_symbol() {
	let graph = create_test_graph();
	let validator = SymbolValidator::new(&graph);
	let result =
		validator.validate_symbol("AuthService");
	assert!(result.exists);
	assert!(result.importance > 0.5);
	assert_eq!(result.definition_count, 1);
	assert_eq!(result.reference_count, 10);
}

#[test]
fn test_validate_nonexistent_symbol() {
	let graph = create_test_graph();
	let validator = SymbolValidator::new(&graph);
	let result =
		validator.validate_symbol("NonExistent");
	assert!(!result.exists);
	assert_eq!(result.importance, 0.0);
}

#[test]
fn test_validate_multiple_symbols() {
	let graph = create_test_graph();
	let validator = SymbolValidator::new(&graph);
	let names = vec![
		"AuthService".to_string(),
		"helper_fn".to_string(),
		"NonExistent".to_string(),
	];
	let result = validator.validate_symbols(&names);
	assert_eq!(result.symbols.len(), 3);
	assert!(
		(result.existence_ratio - 0.666).abs() < 0.01
	);
}

#[test]
fn test_importance_ranking() {
	let graph = create_test_graph();
	let validator = SymbolValidator::new(&graph);
	let auth = validator
		.calculate_importance("AuthService");
	let helper =
		validator.calculate_importance("helper_fn");
	assert!(auth > helper);
}
