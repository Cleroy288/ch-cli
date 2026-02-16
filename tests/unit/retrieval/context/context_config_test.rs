//! Tests for retrieval::context::context_config

use rustean::retrieval::context::ContextConfig;

#[test]
fn test_context_config_defaults() {
	let config = ContextConfig::default();
	assert_eq!(config.max_callers, 15);
	assert_eq!(config.max_callees, 15);
	assert_eq!(config.max_usages_per_symbol, 30);
	assert_eq!(config.context_lines_after, 15);
}

#[test]
fn test_context_config_custom() {
	let config = ContextConfig {
		max_callers: 20,
		max_callees: 20,
		max_usages_per_symbol: 50,
		..ContextConfig::default()
	};
	assert_eq!(config.max_callers, 20);
	assert_eq!(config.max_usages_per_symbol, 50);
}
