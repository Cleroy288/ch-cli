use std::collections::HashMap;

use crate::indexer::symbols::SymbolKind;

use super::types::{Definition, ReferenceContext, SymbolReference};
use super::SemanticGraph;

impl SemanticGraph {
	/// Find where a type is used in signatures, fields, etc.
	/// Returns references that are in type-related contexts
	pub fn find_type_usages(&self, type_name: &str) -> Vec<SymbolReference> {
		self.references_by_name
			.get(type_name)
			.map(|refs| {
				refs.iter()
					.filter(|sym_ref| sym_ref.context.is_type_usage())
					.cloned()
					.collect()
			})
			.unwrap_or_default()
	}

	/// Searches for impl blocks whose signature contains the trait name
	pub fn find_trait_implementations(
		&self,
		trait_name: &str,
	) -> Vec<Definition> {
		self.definitions_by_name
			.values()
			.flatten()
			.filter(|def| {
				def.symbol.kind == SymbolKind::Impl
					&& def
						.symbol
						.signature
						.as_ref()
						.map(|sig| sig.contains(trait_name))
						.unwrap_or(false)
			})
			.cloned()
			.collect()
	}

	/// Returns a map from ReferenceContext to list of references
	pub fn find_usages_by_context(
		&self,
		name: &str,
	) -> HashMap<ReferenceContext, Vec<SymbolReference>> {
		let mut grouped: HashMap<
			ReferenceContext,
			Vec<SymbolReference>,
		> = HashMap::new();

		if let Some(refs) = self.references_by_name.get(name) {
			for reference in refs {
				grouped
					.entry(reference.context)
					.or_default()
					.push(reference.clone());
			}
		}

		grouped
	}
}
