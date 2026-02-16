use rustean::retrieval::hybrid::HybridSearchConfig;

/// Test HybridSearchConfig default values
#[test]
fn test_hybrid_search_config_default() {
	let config = HybridSearchConfig::default();

	assert!((config.keyword_weight - 0.5).abs() < 1e-6);
	assert!((config.semantic_weight - 0.5).abs() < 1e-6);
	assert_eq!(config.candidates_per_source, 50);
	assert!((config.score_threshold - 0.05).abs() < 1e-6);
	assert_eq!(config.min_results_per_type, 1);
}
