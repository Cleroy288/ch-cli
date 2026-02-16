use rustean::retrieval::hybrid::adaptive::{
	classify_query, compute_weights,
	AdaptiveWeights, QueryType,
};

#[test]
fn test_symbol_lookup_weights() {
	let weights = compute_weights("find AuthService");
	assert!(weights.keyword_weight > weights.semantic_weight);
}

#[test]
fn test_conceptual_weights() {
	let weights = compute_weights("how does authentication work");
	assert!(weights.semantic_weight > weights.keyword_weight);
}

#[test]
fn test_definition_weights() {
	let weights = compute_weights("where is AuthService defined");
	assert!(weights.keyword_weight > weights.semantic_weight);
	assert!(weights.keyword_weight >= 1.5);
}

#[test]
fn test_usage_weights() {
	let weights = compute_weights("where is AuthService used");
	assert!(weights.keyword_weight > weights.semantic_weight);
}

#[test]
fn test_mixed_query() {
	let weights = compute_weights("how does AuthService handle login");
	// mixed queries should be balanced
	let ratio = weights.keyword_weight / weights.semantic_weight;
	assert!(ratio > 0.5 && ratio < 2.0);
}

#[test]
fn test_classify_explicit() {
	let qtype = classify_query("find RetrievalPipeline");
	assert_eq!(qtype, QueryType::SymbolLookup);
}

#[test]
fn test_classify_conceptual() {
	let qtype = classify_query("why does the daemon fail");
	assert_eq!(qtype, QueryType::Conceptual);
}

/// Test AdaptiveWeights default produces equal keyword/semantic weights
#[test]
fn test_adaptive_weights_default() {
	let weights = AdaptiveWeights::default(); // create default weights

	assert!((weights.keyword_weight - 1.0).abs() < 1e-6);
	assert!((weights.semantic_weight - 1.0).abs() < 1e-6);
}
