//! Tests for retrieval::agent (mod.rs)

use ch_cli::retrieval::agent::RetrievalOutput;
use ch_cli::retrieval::daemon::protocol::{
	QueryIntent, SearchSpec,
};

#[test]
fn test_has_results_with_results() {
	let output = RetrievalOutput {
		query: "test".to_string(),
		search_spec: SearchSpec {
			original_query: "test".to_string(),
			symbol_names: vec![],
			intent: QueryIntent::Search,
			file_filters: vec![],
			context_hints: vec![],
		},
		xml_output: "".to_string(),
		result_count: 5,
		token_count: 100,
		has_more: false,
	};

	assert!(output.has_results());
}

#[test]
fn test_has_results_without_results() {
	let output = RetrievalOutput {
		query: "test".to_string(),
		search_spec: SearchSpec {
			original_query: "test".to_string(),
			symbol_names: vec![],
			intent: QueryIntent::Search,
			file_filters: vec![],
			context_hints: vec![],
		},
		xml_output: "".to_string(),
		result_count: 0,
		token_count: 0,
		has_more: false,
	};

	assert!(!output.has_results());
}

#[test]
fn test_summary_formatting() {
	let output = RetrievalOutput {
		query: "find daemon".to_string(),
		search_spec: SearchSpec {
			original_query: "find daemon".to_string(),
			symbol_names: vec![
				"DaemonClient".to_string(),
			],
			intent: QueryIntent::FindDefinition,
			file_filters: vec![],
			context_hints: vec![],
		},
		xml_output: "<xml/>".to_string(),
		result_count: 3,
		token_count: 250,
		has_more: true,
	};

	let summary = output.summary();
	assert!(summary.contains("find daemon"));
	assert!(summary.contains("FindDefinition"));
	assert!(summary.contains("DaemonClient"));
	assert!(summary.contains("3"));
	assert!(summary.contains("250"));
}
