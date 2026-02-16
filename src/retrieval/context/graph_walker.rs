//! Graph Walker for Context Expansion
//!
//! Uses SemanticGraph to traverse definitions and references,
//! finding parent scope, related types, and call relationships.

use std::path::Path;

use crate::indexer::{SemanticGraph, Symbol, SymbolKind};

use super::{ContextConfig, ParentContext};
use super::file_reader;
use super::graph_walker_usage::{UsageCollection, UsageInfo};

/// Walks the semantic graph to find context for symbols
pub struct GraphWalker<'graph> {
	/// the semantic graph
	graph: &'graph SemanticGraph,
	/// configuration
	config: ContextConfig,
}

impl<'graph> GraphWalker<'graph> {
	/// Create a new graph walker
	pub fn new(
		graph: &'graph SemanticGraph,
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
		file: &Path,
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

impl<'graph> GraphWalker<'graph> {
	/// Extract a code snippet with context lines.
	/// Uses BufReader to read only the needed line
	/// range instead of loading the full file.
	pub fn extract_snippet(
		&self,
		file: &std::path::Path,
		line: usize,
		context_lines: usize,
	) -> Option<String> {
		// 0-indexed line range to read
		let start =
			line.saturating_sub(context_lines + 1);
		let end = line + context_lines;

		let lines = file_reader::read_lines_range(
			file, start, end,
		).ok()?;

		if lines.is_empty() {
			return None;
		}

		let snippet: String = lines
			.iter()
			.enumerate()
			.map(|(idx, line_str)| {
				format!(
					"{:>4} | {}",
					start + idx + 1, line_str,
				)
			})
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
				containing_symbol: containing
					.map(|sym| sym.name.clone()),
			});
		}

		usages.sort_by_file();
		usages.truncate(self.config.max_usages_per_symbol);

		usages
	}
}

