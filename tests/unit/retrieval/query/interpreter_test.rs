use rustean::retrieval::query::QueryInterpreter;

#[test]
fn test_interpret_simple() {
	let spec = QueryInterpreter::interpret_simple(
		"where is AuthService defined",
	);
	let has_auth = spec
		.symbol_names
		.contains(&"AuthService".to_string());
	assert!(has_auth);
}

#[test]
fn test_interpreter_fallback() {
	let mut interp = QueryInterpreter::new();
	let spec =
		interp.interpret("find the parse_config function");
	// should use fallback since no LLM loaded
	assert!(
		spec.symbol_names
			.contains(&"parse_config".to_string())
	);
}

/// Test has_llm returns false when no model is loaded
#[test]
fn test_has_llm_without_model() {
	let interp = QueryInterpreter::new();

	let has_model = interp.has_llm();

	assert!(!has_model);
}

/// Test default constructor creates interpreter without model
#[test]
fn test_default_has_no_llm() {
	let interp = QueryInterpreter::default();

	let has_model = interp.has_llm();

	assert!(!has_model);
}
