use ch_cli::retrieval::hybrid::HybridSearchConfig;

/// Test HybridSearchConfig default values match expected constants
#[test]
fn test_hybrid_search_config_default() {
	// create config with default values
	let config = HybridSearchConfig::default();

	assert!((config.keyword_weight - 1.0).abs() < 1e-6);
	assert!((config.semantic_weight - 1.0).abs() < 1e-6);
	assert_eq!(config.candidates_per_source, 50);
	assert!((config.rrf_k - 60.0).abs() < 1e-6);
	assert!((config.rrf_score_threshold - 0.015).abs() < 1e-6);
	assert_eq!(config.min_results_per_type, 1);
}
