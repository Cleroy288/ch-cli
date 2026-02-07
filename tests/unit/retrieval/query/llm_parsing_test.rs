use ch_cli::retrieval::daemon::protocol::QueryIntent;
use ch_cli::retrieval::query::{
	fallback_parse, parse_llm_response,
};

#[test]
fn test_fallback_parse() {
	let spec =
		fallback_parse("where is AuthService defined");
	assert!(spec
		.symbol_names
		.contains(&"AuthService".to_string()));
	assert!(matches!(
		spec.intent,
		QueryIntent::FindDefinition
	));
}

#[test]
fn test_parse_llm_response() {
	let json = r#"{"symbols": ["foo", "bar"], "intent": "understand", "file_patterns": ["*.rs"], "hints": []}"#;
	let spec = parse_llm_response("test query", json);
	assert_eq!(spec.symbol_names, vec!["foo", "bar"]);
	assert!(matches!(
		spec.intent,
		QueryIntent::Understand
	));
}

#[test]
fn test_parse_llm_response_filters_stop_words() {
	let json = r#"{"symbols": ["How", "does", \
		"BgeEmbedder", "work"], "intent": "understand", \
		"file_patterns": [], "hints": []}"#;
	let spec = parse_llm_response(
		"How does BgeEmbedder work?",
		json,
	);
	assert_eq!(spec.symbol_names, vec!["BgeEmbedder"]);
	assert!(matches!(
		spec.intent,
		QueryIntent::Understand
	));
}

#[test]
fn test_fallback_parse_extracts_identifiers() {
	let spec = fallback_parse(
		"How does the RetrievalPipeline work?",
	);
	assert!(spec.symbol_names.contains(
		&"RetrievalPipeline".to_string()
	));
	assert!(!spec
		.symbol_names
		.contains(&"How".to_string()));
	assert!(matches!(
		spec.intent,
		QueryIntent::Understand
	));
}
