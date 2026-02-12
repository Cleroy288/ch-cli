//! Triple Fusion — Shared Helpers
//!
//! Score map insertion, symbol keying, and ref count
//! penalty helpers used by the triple fusion pipeline.

use std::collections::HashMap;
use std::sync::Arc;

use crate::indexer::symbols::Symbol;
use crate::retrieval::hybrid::hub_penalty::ref_count_penalty;
use crate::retrieval::hybrid::hub_ref_counts::lookup_ref_count;
use crate::retrieval::hybrid::HybridSearchResult;

/// Min-max bounds for normalization
pub(crate) struct ScoreBoundsRange {
	/// minimum raw score
	pub min: f32,
	/// maximum raw score
	pub max: f32,
}

/// Computed score entry for map insertion
pub(crate) struct ScoreEntry {
	/// fused score to add
	pub score: f32,
	/// rank index (0-based, converted to 1-based)
	pub idx: usize,
	/// raw keyword score or semantic distance
	pub raw: f32,
}

/// Compute ref count penalty for a symbol name
pub(crate) fn hit_penalty(
	name: &str,
	ref_counts: &HashMap<String, usize>,
) -> f32 {
	let ref_cnt = lookup_ref_count(name, ref_counts);
	ref_count_penalty(ref_cnt)
}

/// Insert keyword entry into the scores map
pub(crate) fn insert_keyword_entry(
	scores: &mut HashMap<String, HybridSearchResult>,
	symbol: &Symbol,
	entry: &ScoreEntry,
) {
	let key = symbol_key(symbol);
	let slot = scores
		.entry(key)
		.or_insert_with(|| empty_result(symbol));
	slot.score += entry.score;
	slot.keyword_rank = Some(entry.idx + 1);
	slot.keyword_score = Some(entry.raw);
}

/// Insert semantic entry into the scores map
pub(crate) fn insert_semantic_entry(
	scores: &mut HashMap<String, HybridSearchResult>,
	symbol: &Symbol,
	entry: &ScoreEntry,
) {
	let key = symbol_key(symbol);
	let slot = scores
		.entry(key)
		.or_insert_with(|| empty_result(symbol));
	slot.score += entry.score;
	slot.semantic_rank = Some(entry.idx + 1);
	slot.semantic_distance = Some(entry.raw);
}

/// Create unique key for a symbol (file:line:name)
pub(crate) fn symbol_key(symbol: &Symbol) -> String {
	format!(
		"{}:{}:{}",
		symbol.location.file.display(),
		symbol.location.line,
		symbol.name
	)
}

/// Create empty HybridSearchResult for a symbol
pub(crate) fn empty_result(
	symbol: &Symbol,
) -> HybridSearchResult {
	HybridSearchResult {
		symbol: Arc::new(symbol.clone()),
		score: 0.0,
		keyword_rank: None,
		semantic_rank: None,
		keyword_score: None,
		semantic_distance: None,
		rerank_score: None,
	}
}
