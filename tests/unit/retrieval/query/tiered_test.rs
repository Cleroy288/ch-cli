use ch_cli::retrieval::query::{
	FastPathIntent, FastPathParser, TieredConfig,
};

#[test]
fn test_fast_path_extracts_symbols() {
	let parser = FastPathParser::new();
	let result =
		parser.extract_symbols("find AuthService");
	assert!(!result.symbols.is_empty());
	assert_eq!(result.symbols[0].name, "AuthService");
	assert!(result.use_fast_path);
}

#[test]
fn test_conceptual_query() {
	let parser = FastPathParser::new();
	let result = parser
		.extract_symbols("why does the daemon fail");
	assert_eq!(
		result.intent,
		FastPathIntent::Conceptual
	);
	assert!(!result.use_fast_path);
}

#[test]
fn test_config_default() {
	let config = TieredConfig::default();
	assert!(
		(config.confidence_threshold - 0.7).abs()
			< 0.01
	);
	assert!(
		(config.importance_threshold - 0.5).abs()
			< 0.01
	);
}
