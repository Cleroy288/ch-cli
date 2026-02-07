//! Symbol Importance Scoring
//!
//! Calculates importance scores based on references,
//! visibility, and symbol kind.

use crate::indexer::{SymbolKind, Visibility};

/// Weight for reference count in importance calculation
const REFERENCE_WEIGHT: f32 = 0.4;

/// Weight for visibility in importance calculation
const VISIBILITY_WEIGHT: f32 = 0.3;

/// Weight for symbol kind in importance calculation
const KIND_WEIGHT: f32 = 0.3;

/// Maximum reference count for normalization
const MAX_REFS_NORMALIZE: usize = 50;

/// Calculate visibility score (higher for public symbols)
pub fn visibility_score(visibility: Visibility) -> f32 {
	match visibility {
		Visibility::Public => 1.0,
		Visibility::PublicCrate => 0.7,
		Visibility::PublicSuper => 0.5,
		Visibility::Private => 0.3,
	}
}

/// Calculate kind score (higher for types, lower for fields)
pub fn kind_score(kind: SymbolKind) -> f32 {
	match kind {
		SymbolKind::Struct | SymbolKind::Trait => 1.0,
		SymbolKind::Enum => 0.95,
		SymbolKind::DocumentChunk => 0.85,
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

/// Calculate importance score from references, visibility
/// and kind scores
pub fn calculate_weighted_importance(
	ref_count: usize,
	vis_score: f32,
	k_score: f32,
) -> f32 {
	let ref_score = (ref_count as f32
		/ MAX_REFS_NORMALIZE as f32)
		.min(1.0);

	REFERENCE_WEIGHT * ref_score
		+ VISIBILITY_WEIGHT * vis_score
		+ KIND_WEIGHT * k_score
}

