use ch_cli::retrieval::daemon::DaemonClient;
use ch_cli::retrieval::query::{
	QueryRewriter, decompose_query,
	map_concepts, rewriter_extract_symbols,
};

#[test]
fn test_is_camel_case() {
	let daemon = DaemonClient::new();
	let _rewriter = QueryRewriter::new(&daemon);
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
fn test_decompose() {
	let decomposed = decompose_query(
		"how does the retrieval pipeline work",
	);
	assert!(!decomposed.is_empty());
}
