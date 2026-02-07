//! Tests for retrieval::rerank::cross_encoder

use ch_cli::retrieval::rerank::BgeReranker;

#[test]
#[ignore] // requires model download
fn test_score_single() {
	let reranker = BgeReranker::new().unwrap();
	let score = reranker
		.score(
			"What is machine learning?",
			"Machine learning is a subset of AI.",
		)
		.unwrap();
	assert!(score >= 0.0 && score <= 1.0);
}

#[test]
#[ignore] // requires model download
fn test_score_batch() {
	let reranker = BgeReranker::new().unwrap();
	let docs = vec![
		"Machine learning is a subset of AI."
			.to_string(),
		"The weather is nice today.".to_string(),
	];
	let query = "What is machine learning?";
	let scores =
		reranker.score_batch(query, &docs).unwrap();
	assert_eq!(scores.len(), 2);
	// relevant doc should score higher
	assert!(scores[0] > scores[1]);
}
