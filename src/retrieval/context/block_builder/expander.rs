//! Context Expander
//!
//! Processes search results and expands them into
//! contextual blocks with token budget management.

use crate::indexer::{SemanticGraph, Symbol};

use super::core::BlockBuilder;
use super::super::{ContextConfig, ContextualBlock};

/// Context expander that processes search results
pub struct ContextExpander<'a> {
	/// the block builder
	builder: BlockBuilder<'a>,
	/// max total tokens
	max_tokens: usize,
}

impl<'a> ContextExpander<'a> {
	/// Create a new context expander
	pub fn new(graph: &'a SemanticGraph) -> Self {
		Self {
			builder: BlockBuilder::new(graph),
			max_tokens: 10000,
		}
	}

	/// Create with custom config and token limit
	pub fn with_config(
		graph: &'a SemanticGraph,
		config: ContextConfig,
		max_tokens: usize,
	) -> Self {
		Self {
			builder: BlockBuilder::with_config(graph, config),
			max_tokens,
		}
	}

	/// Expand a list of symbols into contextual blocks
	pub fn expand(
		&self,
		symbols: &[Symbol],
	) -> Vec<ContextualBlock> {
		let mut blocks = Vec::new();
		let mut total_tokens = 0;

		for symbol in symbols {
			match self.builder.build(symbol) {
				Ok(block) => {
					let tokens = block.token_count();
					if total_tokens + tokens > self.max_tokens
					{
						break; // token budget exhausted
					}
					total_tokens += tokens;
					blocks.push(block);
				}
				Err(e) => {
					let msg = format!(
						"context for {}: {}",
						symbol.name, e,
					);
					eprintln!(
						"Warning: failed to build {}",
						msg
					);
				}
			}
		}

		blocks
	}

	/// Expand and format as XML
	pub fn expand_to_xml(&self, symbols: &[Symbol]) -> String {
		let blocks = self.expand(symbols);

		let mut xml = String::from("<context>\n");
		for block in blocks {
			xml.push_str(&block.to_xml());
			xml.push('\n');
		}
		xml.push_str("</context>");

		xml
	}
}
