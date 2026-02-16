//! Symbol Validator
//!
//! Validates extracted symbols against the
//! SemanticGraph. Uses scoring from
//! validator_scoring to rank importance.

use crate::indexer::SemanticGraph;

use super::validator_scoring::{
	calculate_weighted_importance, kind_score,
	visibility_score,
};

/// A validated symbol with existence and importance
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

/// Validator for checking symbols against graph
pub struct SymbolValidator<'graph> {
	/// reference to the semantic graph
	graph: &'graph SemanticGraph,
}

impl<'graph> SymbolValidator<'graph> {
	/// Create a new validator with a graph reference
	pub fn new(
		graph: &'graph SemanticGraph,
	) -> Self {
		Self { graph }
	}

	/// Calculate importance score for a symbol
	#[doc(hidden)]
	pub fn calculate_importance(
		&self, name: &str,
	) -> f32 {
		let definitions =
			self.graph.find_definitions(name);
		let references =
			self.graph.find_references(name);

		if definitions.is_empty() {
			return 0.0;
		}

		let vis = best_visibility(&definitions);
		let kind = best_kind(&definitions);

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
		let references =
			self.graph.find_references(name);
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
			.map(|name| self.validate_symbol(name))
			.collect();

		compute_validation_stats(symbols)
	}
}

/// Get best visibility score from definitions
fn best_visibility(
	definitions: &[&crate::indexer::semantic::Definition],
) -> f32 {
	definitions
		.iter()
		.map(|def| visibility_score(def.symbol.visibility))
		.max_by(|lhs: &f32, rhs: &f32| {
			lhs.partial_cmp(rhs).unwrap()
		})
		.unwrap_or(0.3)
}

/// Get best kind score from definitions
fn best_kind(
	definitions: &[&crate::indexer::semantic::Definition],
) -> f32 {
	definitions
		.iter()
		.map(|def| kind_score(def.symbol.kind))
		.max_by(|lhs: &f32, rhs: &f32| {
			lhs.partial_cmp(rhs).unwrap()
		})
		.unwrap_or(0.3)
}

/// Compute aggregate stats from validated symbols
fn compute_validation_stats(
	symbols: Vec<ValidatedSymbol>,
) -> ValidationResult {
	let existing: Vec<&ValidatedSymbol> = symbols
		.iter()
		.filter(|sym| sym.exists)
		.collect();

	let avg_importance = if existing.is_empty() {
		0.0
	} else {
		existing
			.iter()
			.map(|sym| sym.importance)
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
