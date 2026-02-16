use rustean::retrieval::hybrid::embedding_keywords::{
	build_corpus_stats, extract_keywords,
	format_keywords,
};

#[test]
fn build_corpus_stats_counts_correctly() {
	// Arrange — 3 bodies with overlapping tokens
	let bodies: Vec<&str> = vec![
		"fn parse_query(query: &str) -> Query",
		"fn embed_vector(text: &str) -> Vec<f32>",
		"fn parse_config(path: &str) -> Config",
	];

	// Act
	let stats = build_corpus_stats(&bodies);

	// Assert — "parse" appears in body 0 and 2
	assert_eq!(stats.total_docs, 3);
}

#[test]
fn extract_keywords_returns_discriminative_tokens() {
	// Arrange — "vector" and "embed" only in body 1
	let bodies: Vec<&str> = vec![
		"parse query input token filter",
		"embed vector cosine similarity distance",
		"parse config load settings path",
	];
	let stats = build_corpus_stats(&bodies);

	// Act
	let kws = extract_keywords(bodies[1], &stats);

	// Assert — unique tokens should rank highest
	assert!(kws.contains(&"cosine".to_string()));
	assert!(kws.contains(&"similarity".to_string()));
	assert!(kws.contains(&"vector".to_string()));
}

#[test]
fn extract_keywords_filters_rust_keywords() {
	// Arrange — body full of Rust keywords + one unique
	let bodies: Vec<&str> = vec![
		"fn let mut pub self reindex rebuild",
		"fn let mut pub self other words",
	];
	let stats = build_corpus_stats(&bodies);

	// Act
	let kws = extract_keywords(bodies[0], &stats);

	// Assert — Rust keywords excluded, real words kept
	assert!(!kws.contains(&"pub".to_string()));
	assert!(!kws.contains(&"let".to_string()));
	assert!(kws.contains(&"reindex".to_string()));
}

#[test]
fn extract_keywords_respects_max() {
	// Arrange — body with many unique long tokens
	let body = "alpha bravo charlie delta echo \
		foxtrot golf hotel india juliet";
	let bodies: Vec<&str> = vec![body, "other text here"];
	let stats = build_corpus_stats(&bodies);

	// Act
	let kws = extract_keywords(body, &stats);

	// Assert — never more than 5
	assert!(kws.len() <= 5);
}

#[test]
fn format_keywords_joins_with_spaces() {
	// Arrange
	let kws = vec![
		"embed".to_string(),
		"vector".to_string(),
		"cosine".to_string(),
	];

	// Act
	let result = format_keywords(&kws);

	// Assert
	assert_eq!(result, "embed vector cosine");
}
