use crate::indexer::symbols::Symbol;

use super::graph::SemanticGraph;
use super::types::{Definition, SymbolReference};

impl SemanticGraph {
	pub fn add_symbols(
		&mut self,
		symbols: &[Symbol],
	) {
		for symbol in symbols {
			self.add_definition(symbol.clone());
		}
	}

	pub fn add_definition(
		&mut self,
		symbol: Symbol,
	) {
		let fqn = self.compute_fqn(&symbol);
		let name = symbol.name.clone();
		let file = symbol.location.file.clone();
		let parent = symbol.parent.clone();

		let definition = Definition {
			symbol,
			fqn: fqn.clone(),
			scope: parent.clone(),
		};

		self.definitions_by_file
			.entry(file)
			.or_default()
			.push(definition.clone());

		self.definitions_by_name
			.entry(name)
			.or_default()
			.push(definition);

		if let Some(parent) = parent {
			self.scope_parents
				.insert(fqn, parent);
		}
	}

	pub fn add_reference(
		&mut self,
		reference: SymbolReference,
	) {
		let name = reference.name.clone();
		let file =
			reference.location.file.clone();

		self.references_by_file
			.entry(file)
			.or_default()
			.push(reference.clone());

		self.references_by_name
			.entry(name)
			.or_default()
			.push(reference);
	}

	pub(super) fn compute_fqn(
		&self,
		symbol: &Symbol,
	) -> String {
		match symbol.parent {
			Some(ref parent) => {
				format!("{}::{}", parent, symbol.name)
			}
			None => symbol.name.clone(),
		}
	}
}
