//! Block Builder for Code Extraction
//!
//! Reads source files and extracts code snippets for symbols,
//! assembling them into ContextualBlocks.

use std::fs;

use crate::indexer::{SemanticGraph, Symbol};
use crate::retrieval::RetrievalResult;

use super::{ContextConfig, ContextualBlock, GraphWalker};

/// Builds contextual blocks from symbols
pub struct BlockBuilder<'a> {
	/// the semantic graph
	graph: &'a SemanticGraph,
	/// configuration
	config: ContextConfig,
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
	pub fn with_config(graph: &'a SemanticGraph, config: ContextConfig) -> Self {
		Self { graph, config }
	}

	/// Build a contextual block for a symbol
	/// Uses GraphWalker to collect context and usages with deduplication
	pub fn build(&self, symbol: &Symbol) -> RetrievalResult<ContextualBlock> {
		let code_snippet = self.extract_code(symbol)?; // extracted code for the symbol

		// walk the graph for context
		let walker = GraphWalker::new(self.graph, self.config.clone());

		let parent = walker.find_parent(symbol); // parent scope context
		let callers = walker.find_callers(symbol); // symbols that call this one
		let callees = walker.find_callees(symbol); // symbols called by this one
		let related_types = walker.find_related_types(symbol); // related type info

		// use GraphWalker::find_all_usages for deduplication and sorting
		let usage_collection = walker.find_all_usages(symbol);
		let usage_count = usage_collection.len(); // total usage count
		let usages = usage_collection.into_vec(); // convert to Vec<UsageInfo>

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
	pub fn build_many(&self, symbols: &[Symbol]) -> Vec<RetrievalResult<ContextualBlock>> {
		symbols.iter().map(|s| self.build(s)).collect()
	}

	/// Extract code snippet for a symbol
	fn extract_code(&self, symbol: &Symbol) -> RetrievalResult<String> {
		let path = &symbol.location.file;
		let start_line = symbol.location.line;

		// read the file
		let content = fs::read_to_string(path)?;
		let lines: Vec<&str> = content.lines().collect();

		// calculate line range
		let context_before = self.config.context_lines_before;
		let context_after = self.config.context_lines_after;

		let start = start_line.saturating_sub(context_before + 1);
		let end = find_symbol_end(&lines, start_line - 1, symbol) + context_after;
		let end = end.min(lines.len());

		// extract lines with line numbers
		let mut snippet = String::new();
		for (i, line) in lines[start..end].iter().enumerate() {
			let line_num = start + i + 1;
			snippet.push_str(&format!("{:4} | {}\n", line_num, line));
		}

		Ok(snippet)
	}
}

/// Find the end of a symbol's definition
fn find_symbol_end(lines: &[&str], start_idx: usize, symbol: &Symbol) -> usize {
	// for functions/methods, find matching braces
	// for structs/enums, find closing brace
	// for simple items, use a fixed range

	let mut brace_count = 0;
	let mut found_opening = false;

	for (i, line) in lines[start_idx..].iter().enumerate() {
		for ch in line.chars() {
			match ch {
				'{' => {
					brace_count += 1;
					found_opening = true;
				}
				'}' => {
					brace_count -= 1;
					if found_opening && brace_count == 0 {
						return start_idx + i + 1;
					}
				}
				_ => {}
			}
		}

		// if we've gone too far without finding braces, use a limit
		if i > 100 {
			break;
		}
	}

	// fallback: use line count estimate based on symbol kind
	let estimated_lines = match symbol.kind {
		crate::indexer::SymbolKind::Function | crate::indexer::SymbolKind::Method => 20,
		crate::indexer::SymbolKind::Struct | crate::indexer::SymbolKind::Enum => 15,
		crate::indexer::SymbolKind::Impl => 30,
		crate::indexer::SymbolKind::Trait => 25,
		_ => 5,
	};

	(start_idx + estimated_lines).min(lines.len())
}

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
	pub fn with_config(graph: &'a SemanticGraph, config: ContextConfig, max_tokens: usize) -> Self {
		Self {
			builder: BlockBuilder::with_config(graph, config),
			max_tokens,
		}
	}

	/// Expand a list of symbols into contextual blocks
	pub fn expand(&self, symbols: &[Symbol]) -> Vec<ContextualBlock> {
		let mut blocks = Vec::new();
		let mut total_tokens = 0;

		for symbol in symbols {
			match self.builder.build(symbol) {
				Ok(block) => {
					let tokens = block.token_count();
					if total_tokens + tokens > self.max_tokens {
						break; // token budget exhausted
					}
					total_tokens += tokens;
					blocks.push(block);
				}
				Err(e) => {
					eprintln!("Warning: failed to build context for {}: {}", symbol.name, e);
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::indexer::{CodeLocation, SymbolKind};
	use std::path::PathBuf;

	#[test]
	fn test_find_symbol_end_function() {
		let lines = vec![
			"fn foo() {",
			"    let x = 1;",
			"    x + 1",
			"}",
			"",
			"fn bar() {",
		];

		let symbol = Symbol::new(
			"foo".to_string(),
			SymbolKind::Function,
			CodeLocation::new(PathBuf::from("test.rs"), 1, 1, 0, 0),
		);

		let end = find_symbol_end(&lines, 0, &symbol);
		assert_eq!(end, 4); // after the closing brace
	}
}
