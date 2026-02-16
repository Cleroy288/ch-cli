use rustean::retrieval::query::fast_path_patterns::{
	FastPathIntent, PatternType,
	CONCEPTUAL_PATTERNS, SOURCE_CODE_PATTERNS,
	STOP_WORDS,
};

#[test]
fn test_pattern_type_equality() {
	assert_eq!(
		PatternType::CamelCase,
		PatternType::CamelCase
	);
	assert_ne!(
		PatternType::CamelCase,
		PatternType::SnakeCase
	);
}

#[test]
fn test_fast_path_intent_equality() {
	assert_eq!(
		FastPathIntent::Explicit,
		FastPathIntent::Explicit
	);
	assert_ne!(
		FastPathIntent::Explicit,
		FastPathIntent::Conceptual
	);
}

#[test]
fn test_stop_words_contains_common() {
	assert!(STOP_WORDS.contains(&"how"));
	assert!(STOP_WORDS.contains(&"find"));
	assert!(STOP_WORDS.contains(&"implement"));
	assert!(!STOP_WORDS.contains(&"AuthService"));
}

#[test]
fn test_conceptual_patterns_exist() {
	assert!(
		CONCEPTUAL_PATTERNS.contains(&"how does")
	);
	assert!(CONCEPTUAL_PATTERNS
		.contains(&"architecture of"));
	assert!(!CONCEPTUAL_PATTERNS
		.contains(&"find symbol"));
}

#[test]
fn test_source_code_patterns_exist() {
	assert!(SOURCE_CODE_PATTERNS
		.contains(&"source code"));
	assert!(SOURCE_CODE_PATTERNS
		.contains(&"show me the code"));
}
