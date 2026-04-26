use crate::indexer::symbols::{CodeLocation, Symbol};

use super::types::{
	AllUsages, Definition, ResolutionResult, SymbolReference,
};
use super::SemanticGraph;

impl SemanticGraph {
	pub fn resolve(
		&self,
		reference: &SymbolReference,
	) -> ResolutionResult {
		let definitions: Vec<Definition> = self
			.find_definitions(&reference.name)
			.into_iter()
			.cloned()
			.collect();

		let count = definitions.len();
		let confidence = match count {
			0 => 0.0,
			1 => 1.0,
			n => 0.5 / n as f32 + 0.5,
		};

		ResolutionResult {
			reference: reference.clone(),
			definitions,
			confidence,
		}
	}

	/// Skips full resolution -- looks up definitions
	/// directly to avoid allocating ResolutionResult.
	pub fn validate_reference(
		&self,
		reference: &SymbolReference,
		target: &Symbol,
	) -> bool {
		if reference.name != target.name {
			return false;
		}
		let defs =
			self.find_definitions(&reference.name);
		// 0 defs = external symbol, accept
		// 1 def = unambiguous, accept
		if defs.len() <= 1 {
			return true;
		}
		// multiple: check if any matches target
		defs.iter().any(|def| {
			matches_target_location(def, target)
		})
	}

	pub fn validated_references(
		&self,
		target: &Symbol,
	) -> Vec<&SymbolReference> {
		self.find_references(&target.name)
			.into_iter()
			.filter(|r| self.validate_reference(r, target))
			.collect()
	}

	pub fn find_all_usages(
		&self,
		name: &str,
	) -> AllUsages {
		let definitions: Vec<CodeLocation> = self
			.find_definitions(name)
			.into_iter()
			.map(|def| def.symbol.location.clone())
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

/// Lenient match: exact file+line, or same file+kind
fn matches_target_location(
	def: &Definition,
	target: &Symbol,
) -> bool {
	let same_file = def.symbol.location.file
		== target.location.file;
	if !same_file {
		return false;
	}
	def.symbol.location.line == target.location.line
		|| def.symbol.kind == target.kind
}
