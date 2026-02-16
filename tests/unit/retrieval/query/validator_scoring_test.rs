use rustean::indexer::{SymbolKind, Visibility};
use rustean::retrieval::query::validator_scoring::{
	calculate_weighted_importance, kind_score,
	visibility_score,
};

/// Test visibility_score returns correct values
#[test]
fn test_visibility_score() {
	assert_eq!(
		visibility_score(Visibility::Public),
		1.0
	);
	assert_eq!(
		visibility_score(Visibility::PublicCrate),
		0.7
	);
	assert_eq!(
		visibility_score(Visibility::PublicSuper),
		0.5
	);
	assert_eq!(
		visibility_score(Visibility::Private),
		0.3
	);

	assert!(
		visibility_score(Visibility::Public)
			> visibility_score(Visibility::PublicCrate)
	);
	assert!(
		visibility_score(Visibility::PublicCrate)
			> visibility_score(Visibility::PublicSuper)
	);
	assert!(
		visibility_score(Visibility::PublicSuper)
			> visibility_score(Visibility::Private)
	);
}

/// Test kind_score returns correct values
#[test]
fn test_kind_score() {
	assert_eq!(kind_score(SymbolKind::Struct), 1.0);
	assert_eq!(kind_score(SymbolKind::Trait), 1.0);
	assert_eq!(kind_score(SymbolKind::Enum), 0.95);
	assert_eq!(kind_score(SymbolKind::Function), 0.8);
	assert_eq!(kind_score(SymbolKind::Method), 0.8);
	assert_eq!(kind_score(SymbolKind::Module), 0.7);
	assert_eq!(kind_score(SymbolKind::Field), 0.3);

	assert!(
		kind_score(SymbolKind::Struct)
			> kind_score(SymbolKind::Function)
	);
	assert!(
		kind_score(SymbolKind::Function)
			> kind_score(SymbolKind::Module)
	);
	assert!(
		kind_score(SymbolKind::Module)
			> kind_score(SymbolKind::Constant)
	);
	assert!(
		kind_score(SymbolKind::Constant)
			> kind_score(SymbolKind::Macro)
	);
	assert!(
		kind_score(SymbolKind::Macro)
			> kind_score(SymbolKind::Field)
	);
}

/// Test weighted importance calculation
#[test]
fn test_calculate_weighted_importance() {
	let score = calculate_weighted_importance(
		25, 1.0, 1.0,
	);
	// 0.4 * (25/50) + 0.3 * 1.0 + 0.3 * 1.0
	// = 0.4 * 0.5 + 0.3 + 0.3 = 0.2 + 0.6 = 0.8
	assert!((score - 0.8).abs() < 0.01);

	// Zero refs, private, field
	let low_score = calculate_weighted_importance(
		0, 0.3, 0.3,
	);
	assert!(low_score < score);
}

/// Test ref count is capped at MAX_REFS_NORMALIZE
#[test]
fn test_ref_count_normalization() {
	let score_at_max = calculate_weighted_importance(
		50, 1.0, 1.0,
	);
	let score_over_max = calculate_weighted_importance(
		100, 1.0, 1.0,
	);
	// Both should give same score (capped at 1.0)
	assert!(
		(score_at_max - score_over_max).abs() < 0.01
	);
}
