//! Tests for PRF (Pseudo-Relevance Feedback) module

use rustean::retrieval::query::{
	expand_iterative, expand_with_feedback,
	extract_feedback_terms, prf_split_to_words,
	PrfFeedback, MAX_PRF_ITERATIONS,
};

#[test]
fn expand_with_feedback_adds_terms() {
	// Arrange
	let feedback = PrfFeedback {
		symbol_names: vec![
			"VectorStore".to_string(),
			"VectorSearch".to_string(),
			"VectorIndex".to_string(),
		],
		file_paths: vec![],
	};

	// Act
	let result =
		expand_with_feedback("search code", &feedback);

	// Assert
	assert!(result.is_some());
	let expansion = result.unwrap();
	assert!(expansion.added_terms.contains(
		&"vector".to_string()
	));
	assert!(expansion.expanded_text.contains("vector"));
	assert!(expansion.confidence > 0.0);
}

#[test]
fn expand_with_feedback_skips_existing_terms() {
	// Arrange
	let feedback = PrfFeedback {
		symbol_names: vec![
			"VectorStore".to_string(),
			"VectorSearch".to_string(),
		],
		file_paths: vec![],
	};

	// Act — "vector" is already in the query
	let result = expand_with_feedback(
		"vector search code",
		&feedback,
	);

	// Assert — "vector" should NOT be in added_terms
	if let Some(expansion) = result {
		assert!(!expansion.added_terms.contains(
			&"vector".to_string()
		));
	}
}

#[test]
fn expand_with_feedback_returns_none_on_empty() {
	// Arrange
	let feedback = PrfFeedback {
		symbol_names: vec![],
		file_paths: vec![],
	};

	// Act
	let result =
		expand_with_feedback("search code", &feedback);

	// Assert
	assert!(result.is_none());
}

#[test]
fn extract_feedback_terms_scores_by_frequency() {
	// Arrange — "vector" appears in 3/5, "store" in 1/5
	let feedback = PrfFeedback {
		symbol_names: vec![
			"VectorStore".to_string(),
			"VectorSearch".to_string(),
			"VectorIndex".to_string(),
			"CacheManager".to_string(),
			"QueryParser".to_string(),
		],
		file_paths: vec![],
	};

	// Act
	let terms = extract_feedback_terms(&feedback);

	// Assert — "vector" should score highest (3/5=0.6)
	let vector_score = terms
		.iter()
		.find(|(t, _)| t == "vector")
		.map(|(_, s)| *s);
	let store_score = terms
		.iter()
		.find(|(t, _)| t == "store")
		.map(|(_, s)| *s);

	assert!(vector_score.is_some());
	assert!(
		vector_score.unwrap()
			> store_score.unwrap_or(0.0)
	);
}

#[test]
fn split_to_words_handles_camel_and_snake() {
	// Act — camelCase
	let camel = prf_split_to_words("VectorStore");

	// Assert
	assert_eq!(camel, vec!["vector", "store"]);

	// Act — snake_case
	let snake = prf_split_to_words("vector_store");

	// Assert
	assert_eq!(snake, vec!["vector", "store"]);

	// Act — mixed / short words filtered
	let mixed = prf_split_to_words("MyDBConn");

	// Assert — "my" and "db" are < 3 chars, filtered
	assert_eq!(mixed, vec!["conn"]);
}

#[test]
fn expand_iterative_returns_result_for_feedback() {
	// Arrange
	let feedback = PrfFeedback {
		symbol_names: vec![
			"VectorStore".to_string(),
			"VectorSearch".to_string(),
			"VectorIndex".to_string(),
		],
		file_paths: vec![],
	};

	// Act
	let result =
		expand_iterative("search code", &feedback);

	// Assert
	assert!(result.is_some());
	let expansion = result.unwrap();
	assert!(!expansion.added_terms.is_empty());
	assert!(expansion.expanded_text.contains("vector"));
}

#[test]
fn expand_iterative_returns_none_on_empty() {
	// Arrange
	let feedback = PrfFeedback {
		symbol_names: vec![],
		file_paths: vec![],
	};

	// Act
	let result =
		expand_iterative("search code", &feedback);

	// Assert
	assert!(result.is_none());
}

#[test]
fn expand_iterative_caps_added_terms() {
	// Arrange — many distinct symbols
	let feedback = PrfFeedback {
		symbol_names: vec![
			"AlphaService".to_string(),
			"BetaService".to_string(),
			"GammaService".to_string(),
			"DeltaService".to_string(),
			"EpsilonService".to_string(),
			"ZetaService".to_string(),
			"EtaService".to_string(),
			"ThetaService".to_string(),
		],
		file_paths: vec![],
	};

	// Act
	let result =
		expand_iterative("find logic", &feedback);

	// Assert — should not exceed MAX_PRF_TERMS (5)
	if let Some(expansion) = result {
		assert!(expansion.added_terms.len() <= 5);
	}
}

#[test]
fn max_prf_iterations_is_three() {
	// Assert — constant is set to 3
	assert_eq!(MAX_PRF_ITERATIONS, 3);
}
