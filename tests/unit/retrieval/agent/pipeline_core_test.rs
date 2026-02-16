//! Tests for retrieval::agent::pipeline::core

use std::path::Path;

use rustean::retrieval::agent::pipeline::{
	PipelineConfig, RetrievalPipeline,
};
use rustean::retrieval::docgen::DocStore;

/// Test new creates pipeline with default config
#[test]
fn test_new() {
	let pipeline = RetrievalPipeline::new();

	assert!(pipeline.symbols.is_none());
	assert!(pipeline.graph.is_none());
	assert!(pipeline.hybrid.is_none());
	assert!(pipeline.triple_hybrid.is_none());
	assert!(pipeline.trigram_index.is_none());
	assert!(pipeline.doc_store.is_none());
}

/// Test with_config creates pipeline with custom config
#[test]
fn test_with_config() {
	use rustean::retrieval::agent::pipeline::PipelineFlags;
	let config = PipelineConfig {
		max_results: 50,
		flags: PipelineFlags {
			semantic_search: false,
			..PipelineFlags::default()
		},
		..PipelineConfig::default()
	};

	let pipeline =
		RetrievalPipeline::with_config(config.clone());

	assert_eq!(pipeline.config.max_results, 50);
	assert!(!pipeline.config.flags.semantic_search);
	assert!(pipeline.symbols.is_none());
}

/// Test get_doc_for_symbol returns None when no store
#[test]
fn test_get_doc_for_symbol_no_store() {
	let pipeline = RetrievalPipeline::new();

	let result =
		pipeline.get_doc_for_symbol("test_symbol");
	assert!(result.is_none());
}

/// Test has_doc_context returns false when no store
#[test]
fn test_has_doc_context_no_store() {
	let pipeline = RetrievalPipeline::new();

	assert!(!pipeline.has_doc_context());
}

/// Test has_doc_context false when store not ready
#[test]
fn test_has_doc_context_not_ready() {
	let mut pipeline = RetrievalPipeline::new();
	let store = DocStore::new(Path::new("."));

	pipeline.doc_store = Some(store);

	assert!(!pipeline.has_doc_context());
}
