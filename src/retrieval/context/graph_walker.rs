//! Graph Walker for Context Expansion
//!
//! Uses SemanticGraph to traverse definitions and references,
//! finding parent scope, related types, and call relationships.

use std::path::PathBuf;

use crate::indexer::{SemanticGraph, Symbol, SymbolKind};

use super::{ContextConfig, ParentContext};
use super::graph_walker_usage::{UsageCollection, UsageInfo};

/// Walks the semantic graph to find context for symbols
pub struct GraphWalker<'a> {
	/// the semantic graph
	graph: &'a SemanticGraph,
	/// configuration
	config: ContextConfig,
}

impl<'a> GraphWalker<'a> {
	/// Create a new graph walker
	pub fn new(
		graph: &'a SemanticGraph,
		config: ContextConfig,
	) -> Self {
		Self { graph, config }
	}

	/// Access the semantic graph (for sub-modules)
	pub(super) fn graph(&self) -> &SemanticGraph {
		self.graph
	}

	/// Access the configuration (for sub-modules)
	pub(super) fn config(&self) -> &ContextConfig {
		&self.config
	}

	/// Find the parent context for a symbol
	pub fn find_parent(
		&self,
		symbol: &Symbol,
	) -> Option<ParentContext> {
		if !self.config.include_parent {
			return None;
		}

		let parent_name = match symbol.parent {
			Some(ref name) => name,
			None => return None,
		};

		// look up parent definition
		let parent_defs = self.graph.find_definitions(parent_name);
		if let Some(parent_def) = parent_defs.first() {
			return Some(ParentContext {
				name: parent_name.clone(),
				kind: parent_def.symbol.kind,
				file: parent_def.symbol.location.file.clone(),
				line: parent_def.symbol.location.line,
				snippet: None,
			});
		}

		// parent not found, create basic info
		Some(ParentContext {
			name: parent_name.clone(),
			kind: SymbolKind::Struct,
			file: symbol.location.file.clone(),
			line: 0,
			snippet: None,
		})
	}

	/// Find the symbol that contains a given location
	pub(super) fn find_containing_symbol(
		&self,
		file: &PathBuf,
		line: usize,
	) -> Option<&Symbol> {
		let defs = self.graph.definitions_in_file(file);

		let mut best: Option<&Symbol> = None;
		let mut best_line = 0;

		for def in defs {
			let def_line = def.symbol.location.line;
			if def_line <= line && def_line > best_line {
				best = Some(&def.symbol);
				best_line = def_line;
			}
		}

		best
	}
}

impl<'a> GraphWalker<'a> {
	/// Extract a code snippet with context lines
	pub fn extract_snippet(
		&self,
		file: &std::path::Path,
		line: usize,
		context_lines: usize,
	) -> Option<String> {
		let content = std::fs::read_to_string(file).ok()?;
		let lines: Vec<&str> = content.lines().collect();

		// calculate start and end with bounds checking
		let start = line.saturating_sub(context_lines + 1);
		let end = (line + context_lines).min(lines.len());

		if start >= lines.len() {
			return None;
		}

		let snippet: String = lines[start..end]
			.iter()
			.enumerate()
			.map(|(i, l)| format!("{:>4} | {}", start + i + 1, l))
			.collect::<Vec<_>>()
			.join("\n");

		Some(snippet)
	}

	/// Find all usages of a symbol with full context
	pub fn find_all_usages(
		&self,
		symbol: &Symbol,
	) -> UsageCollection {
		let mut usages = UsageCollection::new();
		let refs = self.graph.find_references(&symbol.name);

		for reference in refs {
			let snippet = self.extract_snippet(
				&reference.location.file,
				reference.location.line,
				2,
			);

			let containing = self.find_containing_symbol(
				&reference.location.file,
				reference.location.line,
			);

			usages.add(UsageInfo {
				file: reference.location.file.clone(),
				line: reference.location.line,
				context: reference.context,
				snippet,
				containing_symbol: containing.map(|s| {
					s.name.clone()
				}),
			});
		}

		usages.sort_by_file();
		usages.truncate(self.config.max_usages_per_symbol);

		usages
	}
}

