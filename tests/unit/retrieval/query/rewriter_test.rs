use rustean::retrieval::query::{
	QueryRewriter, decompose_query,
	map_concepts, rewriter_extract_symbols,
};

#[test]
fn test_is_camel_case() {
	let _rewriter = QueryRewriter::new();
	let symbols = rewriter_extract_symbols(
		"how does RetrievalPipeline work",
	);
	assert!(symbols.contains(
		&"RetrievalPipeline".to_string()
	));
}

#[test]
fn test_is_snake_case() {
	let symbols = rewriter_extract_symbols(
		"find parse_config function",
	);
	assert!(
		symbols.contains(&"parse_config".to_string())
	);
}

#[test]
fn test_extract_symbols() {
	let symbols = rewriter_extract_symbols(
		"how does RetrievalPipeline work",
	);
	assert!(symbols.contains(
		&"RetrievalPipeline".to_string()
	));
}

#[test]
fn test_map_concepts() {
	let mapped =
		map_concepts("how does retrieval work");
	assert!(mapped.contains("implementation"));
	assert!(mapped.contains("retrieve search"));
}

#[test]
fn test_map_concepts_error_handling() {
	let mapped =
		map_concepts("error handling in modules");
	assert!(mapped.contains("result catch"));
}

#[test]
fn test_map_concepts_file_system() {
	let mapped =
		map_concepts("file system operations");
	assert!(mapped.contains("path read write fs"));
}

#[test]
fn test_map_concepts_concurrency() {
	let mapped =
		map_concepts("concurrency patterns");
	assert!(mapped.contains("async await thread"));
}

#[test]
fn test_map_concepts_design_pattern() {
	let mapped = map_concepts("factory pattern");
	assert!(mapped.contains("new create build"));
}

#[test]
fn test_decompose() {
	let decomposed = decompose_query(
		"how does the retrieval pipeline work",
	);
	assert!(!decomposed.is_empty());
}

#[test]
fn test_decompose_expands_camel_case() {
	let result =
		decompose_query("VectorStore search");
	assert!(
		result.contains(&"vector store search".to_string()),
		"Expected 'vector store search', got {:?}",
		result,
	);
}

#[test]
fn test_decompose_expands_snake_case() {
	let result =
		decompose_query("file_watcher implementation");
	assert!(
		result.contains(
			&"file watcher implementation".to_string()
		),
		"Expected 'file watcher implementation', \
			got {:?}",
		result,
	);
}

#[test]
fn test_decompose_expands_mixed_with_pattern() {
	let result =
		decompose_query("how does HybridSearch work");
	// "how does X work" pattern produces X
	assert!(
		result.contains(
			&"HybridSearch".to_string()
		),
		"Expected 'HybridSearch', got {:?}",
		result,
	);
	// identifier expansion produces lowercase words
	assert!(
		result.contains(
			&"how does hybrid search work".to_string()
		),
		"Expected 'how does hybrid search work', \
			got {:?}",
		result,
	);
}

#[test]
fn test_decompose_no_expansion_plain_words() {
	let result =
		decompose_query("error handling");
	// plain words should not produce expansions
	let has_expansion = result.iter().any(|q| {
		q == "error handling"
	});
	assert!(
		!has_expansion,
		"Plain words should not generate expansions, \
			got {:?}",
		result,
	);
}
