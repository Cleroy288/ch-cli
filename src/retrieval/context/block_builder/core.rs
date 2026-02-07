//! Core BlockBuilder Implementation
//!
//! Provides the BlockBuilder struct and basic block building.

use std::fs;

use crate::indexer::{SemanticGraph, Symbol};
use crate::retrieval::RetrievalResult;

use super::super::code_extractor::find_symbol_end;
use super::super::{
	ContextConfig, ContextualBlock, GraphWalker,
};

/// Builds contextual blocks from symbols
pub struct BlockBuilder<'a> {
	/// the semantic graph
	#[doc(hidden)]
	pub graph: &'a SemanticGraph,
	/// configuration
	#[doc(hidden)]
	pub config: ContextConfig,
}

impl<'a> BlockBuilder<'a> {
	/// Create a new block builder
	pub fn new(graph: &'a SemanticGraph) -> Self {
		Self {
			graph,
			config: ContextConfig::default(),
		}
	}

	/// Create with custom configuration
	pub fn with_config(
		graph: &'a SemanticGraph,
		config: ContextConfig,
	) -> Self {
		Self { graph, config }
	}

	/// Build a contextual block for a symbol
	/// Uses GraphWalker to collect context and usages
	pub fn build(
		&self,
		symbol: &Symbol,
	) -> RetrievalResult<ContextualBlock> {
		// extracted code for the symbol
		let code_snippet = self.extract_code(symbol)?;

		// walk the graph for context
		let walker =
			GraphWalker::new(self.graph, self.config.clone());

		let parent = walker.find_parent(symbol);
		let callers = walker.find_callers(symbol);
		let callees = walker.find_callees(symbol);
		let related_types = walker.find_related_types(symbol);

		// use GraphWalker::find_all_usages
		let usage_collection = walker.find_all_usages(symbol);
		let usage_count = usage_collection.len();
		let usages = usage_collection.into_vec();

		Ok(ContextualBlock {
			symbol: symbol.clone(),
			code_snippet,
			parent,
			related_types,
			callers,
			callees,
			doc_comment: symbol.doc_comment.clone(),
			usage_count,
			usages,
		})
	}

	/// Build contextual blocks for multiple symbols
	pub fn build_many(
		&self,
		symbols: &[Symbol],
	) -> Vec<RetrievalResult<ContextualBlock>> {
		symbols.iter().map(|s| self.build(s)).collect()
	}
}
