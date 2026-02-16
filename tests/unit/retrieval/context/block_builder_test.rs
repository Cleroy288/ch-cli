//! Tests for retrieval::context::block_builder

use rustean::indexer::SemanticGraph;
use rustean::retrieval::context::block_builder::BlockBuilder;
use rustean::retrieval::context::ContextConfig;

/// Verify BlockBuilder::with_config stores config
#[test]
fn test_block_builder_with_config() {
	let graph = SemanticGraph::new();
	let config = ContextConfig {
		max_callers: 5,
		max_callees: 10,
		max_usages_per_symbol: 20,
		context_lines_before: 2,
		context_lines_after: 8,
		include_parent: false,
		include_related_types: false,
	};

	let builder =
		BlockBuilder::with_config(&graph, config);

	// verify custom config values are stored
	assert_eq!(builder.config.max_callers, 5);
	assert_eq!(builder.config.max_callees, 10);
	assert_eq!(
		builder.config.max_usages_per_symbol,
		20,
	);
	assert_eq!(
		builder.config.context_lines_before,
		2,
	);
	assert_eq!(
		builder.config.context_lines_after,
		8,
	);
	assert!(!builder.config.include_parent);
	assert!(!builder.config.include_related_types);
}
