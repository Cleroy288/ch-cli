//! Symbol Validator
//!
//! Validates extracted symbols against the SemanticGraph.
//! Uses scoring from validator_scoring to rank importance.

use crate::indexer::SemanticGraph;

use super::validator_scoring::{
	calculate_weighted_importance, kind_score,
	visibility_score,
};

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
	/// Create a new validator with a graph reference
	pub fn new(graph: &'a SemanticGraph) -> Self {
		Self { graph }
	}

	/// Calculate importance score for a symbol
	#[doc(hidden)]
	pub fn calculate_importance(&self, name: &str) -> f32 {
		let definitions =
			self.graph.find_definitions(name);
		let references = self.graph.find_references(name);

		if definitions.is_empty() {
			return 0.0;
		}

		let vis = definitions
			.iter()
			.map(|d| visibility_score(d.symbol.visibility))
			.max_by(|a, b| a.partial_cmp(b).unwrap())
			.unwrap_or(0.3);

		let kind = definitions
			.iter()
			.map(|d| kind_score(d.symbol.kind))
			.max_by(|a, b| a.partial_cmp(b).unwrap())
			.unwrap_or(0.3);

		calculate_weighted_importance(
			references.len(),
			vis,
			kind,
		)
	}

	/// Validate a single symbol name
	pub fn validate_symbol(
		&self,
		name: &str,
	) -> ValidatedSymbol {
		let definitions =
			self.graph.find_definitions(name);
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
	pub fn validate_symbols(
		&self,
		names: &[String],
	) -> ValidationResult {
		let symbols: Vec<ValidatedSymbol> = names
			.iter()
			.map(|n| self.validate_symbol(n))
			.collect();

		let existing: Vec<&ValidatedSymbol> =
			symbols.iter().filter(|s| s.exists).collect();

		let avg_importance = if existing.is_empty() {
			0.0
		} else {
			existing
				.iter()
				.map(|s| s.importance)
				.sum::<f32>()
				/ existing.len() as f32
		};

		let existence_ratio = if symbols.is_empty() {
			0.0
		} else {
			existing.len() as f32
				/ symbols.len() as f32
		};

		ValidationResult {
			symbols,
			avg_importance,
			existence_ratio,
		}
	}
}

