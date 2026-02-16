//! Core BlockBuilder Implementation
//!
//! Provides the BlockBuilder struct and basic block building.

use crate::indexer::{SemanticGraph, Symbol};
use crate::retrieval::RetrievalResult;

use super::super::{
	ContextConfig, ContextualBlock, GraphWalker,
};

/// Builds contextual blocks from symbols
pub struct BlockBuilder<'graph> {
	/// the semantic graph
	#[doc(hidden)]
	pub graph: &'graph SemanticGraph,
	/// configuration
	#[doc(hidden)]
	pub config: ContextConfig,
}

impl<'graph> BlockBuilder<'graph> {
	/// Create a new block builder
	pub fn new(graph: &'graph SemanticGraph) -> Self {
		Self {
			graph,
			config: ContextConfig::default(),
		}
	}

	/// Create with custom configuration
	pub fn with_config(
		graph: &'graph SemanticGraph,
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
		symbols.iter().map(|sym| self.build(sym)).collect()
	}
}
