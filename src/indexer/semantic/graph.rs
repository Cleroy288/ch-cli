//! Core graph manipulation methods for SemanticGraph.

use crate::indexer::symbols::Symbol;

use super::types::{Definition, SymbolReference};
use super::SemanticGraph;

impl SemanticGraph {
	/// Add symbols from parsing as definitions
	pub fn add_symbols(&mut self, symbols: &[Symbol]) {
		for symbol in symbols {
			self.add_definition(symbol.clone());
		}
	}

	/// Add a single definition
	pub fn add_definition(&mut self, symbol: Symbol) {
		let fqn = self.compute_fqn(&symbol); // fully qualified name
		let scope = symbol.parent.clone(); // parent scope
		let file = symbol.location.file.clone(); // file path

		let definition = Definition {
			symbol: symbol.clone(),
			fqn: fqn.clone(),
			scope,
		};

		// Index by name
		self.definitions_by_name
			.entry(symbol.name.clone())
			.or_default()
			.push(definition.clone());

		// Index by file
		self.definitions_by_file
			.entry(file)
			.or_default()
			.push(definition);

		// Track scope hierarchy
		if let Some(ref parent) = symbol.parent {
			self.scope_parents.insert(fqn, parent.clone());
		}
	}

	/// Add a reference
	pub fn add_reference(&mut self, reference: SymbolReference) {
		let file = reference.location.file.clone(); // file path
		let name = reference.name.clone(); // symbol name

		self.references_by_name
			.entry(name)
			.or_default()
			.push(reference.clone());

		self.references_by_file
			.entry(file)
			.or_default()
			.push(reference);
	}

	/// Compute fully qualified name for a symbol
	pub(super) fn compute_fqn(&self, symbol: &Symbol) -> String {
		if let Some(ref parent) = symbol.parent {
			format!("{}::{}", parent, symbol.name)
		} else {
			symbol.name.clone()
		}
	}
}
