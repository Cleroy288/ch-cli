//! Tests for retrieval::context::graph_walker_types

use ch_cli::retrieval::context::graph_walker_types::{
	extract_types_from_signature,
};

#[test]
fn test_extract_types() {
	let sig = "fn parse(input: &str) \
		-> Result<Config, ParseError>";
	let types = extract_types_from_signature(sig);

	assert!(types.contains(&"Config".to_string()));
	assert!(
		types.contains(&"ParseError".to_string())
	);
	assert!(!types.contains(&"Result".to_string()));
}
