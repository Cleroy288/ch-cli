//! Lookup methods for finding definitions and references.

use crate::indexer::symbols::SymbolKind;

use super::types::{Definition, SymbolReference};
use super::SemanticGraph;

impl SemanticGraph {
	/// Find the definition(s) for a symbol name
	pub fn find_definitions(&self, name: &str) -> Vec<&Definition> {
		self.definitions_by_name
			.get(name)
			.map(|defs| defs.iter().collect())
			.unwrap_or_default()
	}

	/// Find all references to a symbol name
	pub fn find_references(&self, name: &str) -> Vec<&SymbolReference> {
		self.references_by_name
			.get(name)
			.map(|refs| refs.iter().collect())
			.unwrap_or_default()
	}

	/// Get all unique symbol names
	pub fn all_symbol_names(&self) -> Vec<&String> {
		self.definitions_by_name.keys().collect()
	}

	/// Find definitions by kind
	pub fn find_by_kind(&self, kind: SymbolKind) -> Vec<&Definition> {
		self.definitions_by_name
			.values()
			.flatten()
			.filter(|d| d.symbol.kind == kind)
			.collect()
	}
}
