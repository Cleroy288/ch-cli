//! Reference resolution methods for SemanticGraph.

use crate::indexer::symbols::{CodeLocation, Symbol};

use super::types::{AllUsages, Definition, ResolutionResult, SymbolReference};
use super::SemanticGraph;

impl SemanticGraph {
	/// Resolve a reference to its possible definitions
	pub fn resolve(&self, reference: &SymbolReference) -> ResolutionResult {
		let definitions: Vec<Definition> = self
			.find_definitions(&reference.name)
			.into_iter()
			.cloned()
			.collect();

		let confidence = if definitions.len() == 1 {
			1.0
		} else if definitions.is_empty() {
			0.0
		} else {
			// Multiple definitions - lower confidence
			0.5 / definitions.len() as f32 + 0.5
		};

		ResolutionResult {
			reference: reference.clone(),
			definitions,
			confidence,
		}
	}

	/// Validate reference refers to target symbol
	/// Returns true if the reference resolves to the target definition
	pub fn validate_reference(
		&self,
		reference: &SymbolReference,
		target: &Symbol
	) -> bool {
		// name must match
		if reference.name != target.name {
			return false;
		}

		// resolve to find all possible definitions
		let resolution = self.resolve(reference);

		// if no definitions found, accept the reference (external symbol)
		if resolution.definitions.is_empty() {
			return true;
		}

		// if only one definition, accept (unambiguous)
		if resolution.definitions.len() == 1 {
			return true;
		}

		// multiple definitions - check if any matches the target location
		// be lenient: match by file only if line doesn't match exactly
		resolution.definitions.iter().any(|def| {
			// exact match
			if def.symbol.location.file == target.location.file
				&& def.symbol.location.line == target.location.line
			{
				return true;
			}

			// same file match for fields/methods
			// (might have different line tracking)
			if def.symbol.location.file == target.location.file
				&& def.symbol.kind == target.kind
			{
				return true;
			}

			false
		})
	}

	/// Filter references to only those that resolve to a specific target
	pub fn validated_references(&self, target: &Symbol) -> Vec<&SymbolReference> {
		self.find_references(&target.name)
			.into_iter()
			.filter(|r| self.validate_reference(r, target))
			.collect()
	}

	/// Find all usages of a symbol (definition + references)
	pub fn find_all_usages(&self, name: &str) -> AllUsages {
		let definitions: Vec<CodeLocation> = self
			.find_definitions(name)
			.into_iter()
			.map(|d| d.symbol.location.clone())
			.collect();

		let references: Vec<CodeLocation> = self
			.find_references(name)
			.into_iter()
			.map(|r| r.location.clone())
			.collect();

		AllUsages {
			name: name.to_string(),
			definitions,
			references,
		}
	}
}
