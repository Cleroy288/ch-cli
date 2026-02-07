use ch_cli::retrieval::daemon::protocol::QueryIntent;
use ch_cli::retrieval::query::{
	tiered_detect_intent, build_fast_path_spec_impl,
};

#[test]
fn test_detect_intent_find_definition() {
	let intent =
		tiered_detect_intent("where is AuthService defined");
	assert!(matches!(
		intent,
		QueryIntent::FindDefinition
	));
}

#[test]
fn test_detect_intent_find_usages() {
	let intent =
		tiered_detect_intent("who calls AuthService");
	assert!(matches!(
		intent,
		QueryIntent::FindUsages
	));
}

#[test]
fn test_detect_intent_understand() {
	let intent = tiered_detect_intent(
		"explain how authentication works",
	);
	assert!(matches!(intent, QueryIntent::Understand));
}

#[test]
fn test_detect_intent_search() {
	let intent =
		tiered_detect_intent("find AuthService");
	assert!(matches!(intent, QueryIntent::Search));
}

#[test]
fn test_build_fast_path_spec() {
	let query = "find AuthService";
	let symbols = vec!["AuthService".to_string()];
	let spec =
		build_fast_path_spec_impl(query, symbols);

	assert_eq!(
		spec.original_query,
		"find AuthService"
	);
	assert_eq!(spec.symbol_names.len(), 1);
	assert_eq!(spec.symbol_names[0], "AuthService");
	assert!(spec.file_filters.is_empty());
	assert!(spec.context_hints.is_empty());
}
