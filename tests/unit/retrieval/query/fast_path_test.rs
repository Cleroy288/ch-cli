use rustean::retrieval::query::fast_path_patterns::{
	FastPathIntent, PatternType,
};
use rustean::retrieval::query::FastPathParser;

#[test]
fn test_extract_camel_case() {
	let parser = FastPathParser::new();
	let result =
		parser.extract_symbols("find AuthService");
	assert_eq!(result.symbols.len(), 1);
	assert_eq!(result.symbols[0].name, "AuthService");
	assert_eq!(
		result.symbols[0].pattern_type,
		PatternType::CamelCase
	);
	assert!(result.use_fast_path);
}

#[test]
fn test_extract_snake_case() {
	let parser = FastPathParser::new();
	let result = parser
		.extract_symbols("where is parse_config");
	assert_eq!(result.symbols.len(), 1);
	assert_eq!(
		result.symbols[0].name,
		"parse_config"
	);
}

#[test]
fn test_conceptual_query() {
	let parser = FastPathParser::new();
	let result = parser.extract_symbols(
		"why does the daemon fail to start",
	);
	assert_eq!(
		result.intent,
		FastPathIntent::Conceptual
	);
	assert!(!result.use_fast_path);
}

#[test]
fn test_mixed_query() {
	let parser = FastPathParser::new();
	let result = parser.extract_symbols(
		"how does AuthService handle tokens",
	);
	assert_eq!(result.intent, FastPathIntent::Mixed);
	assert!(!result.use_fast_path);
}

#[test]
fn test_multiple_symbols() {
	let parser = FastPathParser::new();
	let result = parser
		.extract_symbols("BgeEmbedder and HybridSearch");
	assert_eq!(result.symbols.len(), 2);
	assert!(result.use_fast_path);
}
