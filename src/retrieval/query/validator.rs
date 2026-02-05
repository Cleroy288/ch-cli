//! Symbol Validator
//!
//! Validates extracted symbols against the SemanticGraph.
//! Calculates importance scores based on references, visibility, and kind.

use crate::indexer::{SemanticGraph, SymbolKind, Visibility};

/// Weights for importance score calculation
const REFERENCE_WEIGHT: f32 = 0.4;
const VISIBILITY_WEIGHT: f32 = 0.3;
const KIND_WEIGHT: f32 = 0.3;

/// Maximum reference count for normalization
const MAX_REFS_NORMALIZE: usize = 50;

/// A validated symbol with existence and importance info
#[derive(Debug, Clone)]
pub struct ValidatedSymbol {
	/// the symbol name
	pub name: String,
	/// whether the symbol exists in the graph
	pub exists: bool,
	/// importance score (0.0 - 1.0)
	pub importance: f32,
	/// number of definitions found
	pub definition_count: usize,
	/// number of references found
	pub reference_count: usize,
}

/// Result of symbol validation
#[derive(Debug, Clone)]
pub struct ValidationResult {
	/// validated symbols
	pub symbols: Vec<ValidatedSymbol>,
	/// average importance of existing symbols
	pub avg_importance: f32,
	/// proportion of symbols that exist
	pub existence_ratio: f32,
}

/// Validator for checking symbols against SemanticGraph
pub struct SymbolValidator<'a> {
	/// reference to the semantic graph
	graph: &'a SemanticGraph,
}

impl<'a> SymbolValidator<'a> {
	/// Create a new validator with a SemanticGraph reference
	pub fn new(graph: &'a SemanticGraph) -> Self {
		Self { graph }
	}

	/// Calculate visibility score (higher for public symbols)
	fn visibility_score(&self, visibility: Visibility) -> f32 {
		match visibility {
			Visibility::Public => 1.0,
			Visibility::PublicCrate => 0.7,
			Visibility::PublicSuper => 0.5,
			Visibility::Private => 0.3,
		}
	}

	/// Calculate kind score (higher for types, lower for fields)
	fn kind_score(&self, kind: SymbolKind) -> f32 {
		match kind {
			SymbolKind::Struct | SymbolKind::Trait => 1.0,
			SymbolKind::Enum => 0.95,
			SymbolKind::DocumentChunk => 0.85, // docs important for conceptual queries
			SymbolKind::Function | SymbolKind::Method => 0.8,
			SymbolKind::Impl => 0.75,
			SymbolKind::Module => 0.7,
			SymbolKind::Constant | SymbolKind::Static => 0.6,
			SymbolKind::TypeAlias => 0.55,
			SymbolKind::Macro => 0.5,
			SymbolKind::EnumVariant => 0.4,
			SymbolKind::Field => 0.3,
		}
	}

	/// Calculate importance score for a symbol
	fn calculate_importance(&self, name: &str) -> f32 {
		let definitions = self.graph.find_definitions(name);
		let references = self.graph.find_references(name);

		if definitions.is_empty() {
			return 0.0;
		}

		// Reference score: normalized by MAX_REFS_NORMALIZE
		let ref_count = references.len();
		let ref_score = (ref_count as f32 / MAX_REFS_NORMALIZE as f32).min(1.0);

		// Visibility score: use highest visibility among definitions
		let vis_score = definitions
			.iter()
			.map(|d| self.visibility_score(d.symbol.visibility))
			.max_by(|a, b| a.partial_cmp(b).unwrap())
			.unwrap_or(0.3);

		// Kind score: use highest kind score among definitions
		let kind_score = definitions
			.iter()
			.map(|d| self.kind_score(d.symbol.kind))
			.max_by(|a, b| a.partial_cmp(b).unwrap())
			.unwrap_or(0.3);

		// Weighted combination
		REFERENCE_WEIGHT * ref_score + VISIBILITY_WEIGHT * vis_score + KIND_WEIGHT * kind_score
	}

	/// Validate a single symbol name
	pub fn validate_symbol(&self, name: &str) -> ValidatedSymbol {
		let definitions = self.graph.find_definitions(name);
		let references = self.graph.find_references(name);
		let exists = !definitions.is_empty();
		let importance = if exists {
			self.calculate_importance(name)
		} else {
			0.0
		};

		ValidatedSymbol {
			name: name.to_string(),
			exists,
			importance,
			definition_count: definitions.len(),
			reference_count: references.len(),
		}
	}

	/// Validate multiple symbol names
	pub fn validate_symbols(&self, names: &[String]) -> ValidationResult {
		let symbols: Vec<ValidatedSymbol> =
			names.iter().map(|n| self.validate_symbol(n)).collect();

		let existing: Vec<&ValidatedSymbol> = symbols.iter().filter(|s| s.exists).collect();

		let avg_importance = if existing.is_empty() {
			0.0
		} else {
			existing.iter().map(|s| s.importance).sum::<f32>() / existing.len() as f32
		};

		let existence_ratio = if symbols.is_empty() {
			0.0
		} else {
			existing.len() as f32 / symbols.len() as f32
		};

		ValidationResult {
			symbols,
			avg_importance,
			existence_ratio,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::indexer::{CodeLocation, Symbol};
	use std::path::PathBuf;

	fn create_test_graph() -> SemanticGraph {
		let mut graph = SemanticGraph::new();

		// Add a public struct with references
		let auth = Symbol::new(
			"AuthService".to_string(),
			SymbolKind::Struct,
			CodeLocation::new(PathBuf::from("src/auth.rs"), 10, 1, 0, 11),
		)
		.with_visibility(Visibility::Public);
		graph.add_definition(auth);

		// Add references to AuthService
		for i in 0..10 {
			graph.add_reference(crate::indexer::SymbolReference {
				name: "AuthService".to_string(),
				location: CodeLocation::new(PathBuf::from("src/main.rs"), 20 + i, 1, 0, 11),
				context: crate::indexer::ReferenceContext::Type,
			});
		}

		// Add a private function
		let helper = Symbol::new(
			"helper_fn".to_string(),
			SymbolKind::Function,
			CodeLocation::new(PathBuf::from("src/utils.rs"), 5, 1, 0, 9),
		);
		graph.add_definition(helper);

		graph
	}

	#[test]
	fn test_validate_existing_symbol() {
		let graph = create_test_graph();
		let validator = SymbolValidator::new(&graph);

		let result = validator.validate_symbol("AuthService");
		assert!(result.exists);
		assert!(result.importance > 0.5);
		assert_eq!(result.definition_count, 1);
		assert_eq!(result.reference_count, 10);
	}

	#[test]
	fn test_validate_nonexistent_symbol() {
		let graph = create_test_graph();
		let validator = SymbolValidator::new(&graph);

		let result = validator.validate_symbol("NonExistent");
		assert!(!result.exists);
		assert_eq!(result.importance, 0.0);
	}

	#[test]
	fn test_validate_multiple_symbols() {
		let graph = create_test_graph();
		let validator = SymbolValidator::new(&graph);

		let names = vec![
			"AuthService".to_string(),
			"helper_fn".to_string(),
			"NonExistent".to_string(),
		];
		let result = validator.validate_symbols(&names);

		assert_eq!(result.symbols.len(), 3);
		assert!((result.existence_ratio - 0.666).abs() < 0.01);
	}

	#[test]
	fn test_importance_ranking() {
		let graph = create_test_graph();
		let validator = SymbolValidator::new(&graph);

		// AuthService (public struct with refs) should rank higher than helper_fn (private fn)
		let auth_importance = validator.calculate_importance("AuthService");
		let helper_importance = validator.calculate_importance("helper_fn");

		assert!(auth_importance > helper_importance);
	}
}
