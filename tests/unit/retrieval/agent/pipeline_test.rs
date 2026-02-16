//! Tests for retrieval::agent::pipeline (mod.rs)

use rustean::retrieval::agent::pipeline::{
	PipelineConfig, RetrievalPipeline,
};
use rustean::retrieval::daemon::protocol::QueryIntent;

#[test]
fn test_config_default() {
	let config = PipelineConfig::default();
	assert_eq!(config.max_results, 20);
	assert!(config.flags.expand_query);
	assert!(config.flags.semantic_search);
	assert!(config.flags.use_daemon_cache);
	assert!(config.flags.include_usage_counts);
}

#[test]
fn test_config_persistence_enabled_by_default() {
	let config = PipelineConfig::default();
	assert!(
		config.flags.enable_persistence,
		"Persistence should be enabled by default"
	);
}

#[test]
fn test_config_with_persistence_disabled() {
	use rustean::retrieval::agent::pipeline::PipelineFlags;
	let config = PipelineConfig {
		flags: PipelineFlags {
			enable_persistence: false,
			..PipelineFlags::default()
		},
		..PipelineConfig::default()
	};
	assert!(!config.flags.enable_persistence);
}

#[test]
fn test_fallback_query_expansion_extracts_symbols() {
	let pipeline = RetrievalPipeline::new();

	// Test CamelCase extraction
	let spec = pipeline
		.fallback_query_expansion("find AuthService");
	assert!(
		spec.symbol_names
			.contains(&"AuthService".to_string()),
		"Should extract CamelCase symbol"
	);

	// Test snake_case extraction
	let spec2 = pipeline.fallback_query_expansion(
		"where is parse_config",
	);
	assert!(
		spec2.symbol_names
			.contains(&"parse_config".to_string()),
		"Should extract snake_case symbol"
	);

	// Test multiple symbols
	let query3 =
		"RetrievalPipeline and HybridSearch";
	let spec3 =
		pipeline.fallback_query_expansion(query3);
	assert!(
		spec3.symbol_names.contains(
			&"RetrievalPipeline".to_string()
		),
		"Should extract first CamelCase symbol"
	);
	assert!(
		spec3.symbol_names
			.contains(&"HybridSearch".to_string()),
		"Should extract second CamelCase symbol"
	);
}

#[test]
fn test_fallback_query_expansion_detects_intent() {
	let pipeline = RetrievalPipeline::new();

	// Test definition intent
	let spec_def = pipeline.fallback_query_expansion(
		"where is AuthService defined",
	);
	assert!(
		matches!(
			spec_def.intent,
			QueryIntent::FindDefinition
		),
		"Should detect FindDefinition intent"
	);

	// Test usage intent
	let spec_usage = pipeline
		.fallback_query_expansion(
			"how is parse_config used",
		);
	assert!(
		matches!(
			spec_usage.intent,
			QueryIntent::FindUsages
		),
		"Should detect FindUsages intent"
	);

	// Test understand intent
	let spec_understand = pipeline
		.fallback_query_expansion(
			"how does the pipeline work",
		);
	assert!(
		matches!(
			spec_understand.intent,
			QueryIntent::Understand
		),
		"Should detect Understand intent"
	);

	// Test debug intent
	let spec_debug = pipeline
		.fallback_query_expansion(
			"debug error in AuthService",
		);
	assert!(
		matches!(
			spec_debug.intent,
			QueryIntent::Debug
		),
		"Should detect Debug intent"
	);

	// Test default search intent
	let spec_search = pipeline
		.fallback_query_expansion("AuthService");
	assert!(
		matches!(
			spec_search.intent,
			QueryIntent::Search
		),
		"Should default to Search intent"
	);
}
