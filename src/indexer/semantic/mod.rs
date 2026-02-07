//! Semantic analysis module for name resolution.
//!
//! This module provides:
//! - Definition tracking (where symbols are declared)
//! - Reference tracking (where symbols are used)
//! - "Go to definition" functionality
//! - "Find all references" functionality
//!
//! This is a simplified implementation that can be enhanced with
//! full stack-graphs support in the future.

mod graph;
mod location;
mod lookup;
mod lookup_advanced;
mod resolution;
mod types;

use std::path::PathBuf;

use fxhash::FxHashMap;

// Re-export public types
pub use types::{
	AllUsages, Definition, ReferenceContext, ResolutionResult, SemanticStats, SymbolReference,
};

use types::Definition as Def;
use types::SymbolReference as SymRef;

/// The semantic graph for a project
#[derive(Clone)]
pub struct SemanticGraph {
	/// All definitions indexed by name
	definitions_by_name: FxHashMap<String, Vec<Def>>,
	/// All definitions indexed by file
	definitions_by_file: FxHashMap<PathBuf, Vec<Def>>,
	/// All references indexed by name
	references_by_name: FxHashMap<String, Vec<SymRef>>,
	/// All references indexed by file
	references_by_file: FxHashMap<PathBuf, Vec<SymRef>>,
	/// Scope hierarchy (child -> parent)
	scope_parents: FxHashMap<String, String>,
}

impl SemanticGraph {
	/// Create a new empty semantic graph
	pub fn new() -> Self {
		Self {
			definitions_by_name: FxHashMap::default(),
			definitions_by_file: FxHashMap::default(),
			references_by_name: FxHashMap::default(),
			references_by_file: FxHashMap::default(),
			scope_parents: FxHashMap::default(),
		}
	}

	/// Get statistics about the semantic graph
	pub fn stats(&self) -> SemanticStats {
		SemanticStats {
			total_definitions:
				self.definitions_by_name.values()
				.map(|v| v.len()).sum(),
			total_references:
				self.references_by_name.values()
				.map(|v| v.len()).sum(),
			unique_symbols: self.definitions_by_name.len(),
			files_analyzed: self.definitions_by_file.len(),
		}
	}
}

impl Default for SemanticGraph {
	fn default() -> Self {
		Self::new()
	}
}
