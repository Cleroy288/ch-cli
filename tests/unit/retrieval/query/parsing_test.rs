use ch_cli::retrieval::daemon::protocol::QueryIntent;
use ch_cli::retrieval::query::parsing::{
	contains_word, extract_identifiers,
	filter_stop_words, parse_intent,
};

#[test]
fn test_parse_intent() {
	assert!(matches!(
		parse_intent("find_definition"),
		QueryIntent::FindDefinition
	));
	assert!(matches!(
		parse_intent("usages"),
		QueryIntent::FindUsages
	));
	assert!(matches!(
		parse_intent("understand"),
		QueryIntent::Understand
	));
	assert!(matches!(
		parse_intent("random"),
		QueryIntent::Search
	));
}

#[test]
fn test_filter_stop_words() {
	let input = vec![
		"How".to_string(),
		"does".to_string(),
		"AuthService".to_string(),
		"work".to_string(),
	];
	let filtered = filter_stop_words(input);
	assert_eq!(filtered, vec!["AuthService"]);

	let input2 = vec![
		"a".to_string(),
		"b".to_string(),
		"Parser".to_string(),
	];
	let filtered2 = filter_stop_words(input2);
	assert_eq!(filtered2, vec!["Parser"]);
}

#[test]
fn test_extract_identifiers() {
	let ids = extract_identifiers(
		"How does AuthService work?",
	);
	assert!(ids.contains(&"AuthService".to_string()));
	assert!(!ids.contains(&"How".to_string()));

	let ids2 = extract_identifiers(
		"find parse_config function",
	);
	assert!(ids2.contains(&"parse_config".to_string()));
}

#[test]
fn test_contains_word_whole_words_only() {
	assert!(contains_word(
		"how does this work",
		"how"
	));
	assert!(contains_word("tell me how", "how"));
	assert!(!contains_word(
		"the howitzer fires",
		"how"
	));
	assert!(!contains_word(
		"somehow it works",
		"how"
	));
	assert!(contains_word("how", "how"));
	assert!(!contains_word("show", "how"));
}
