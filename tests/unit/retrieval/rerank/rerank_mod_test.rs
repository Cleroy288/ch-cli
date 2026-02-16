//! Tests for retrieval::rerank (mod.rs)

use rustean::retrieval::rerank::{
	rerank_by_score, RerankedItem,
};

/// Verify rerank_by_score sorts items by descending score
#[test]
fn test_rerank_by_score() {
	let items = vec![
		RerankedItem { item: "low", score: 0.2, original_rank: 0 },
		RerankedItem { item: "high", score: 0.9, original_rank: 1 },
		RerankedItem { item: "mid", score: 0.5, original_rank: 2 },
	]; // items in unsorted order

	let ranked = rerank_by_score(items); // rerank by score descending

	// verify descending score order
	assert_eq!(ranked[0].item, "high");
	assert_eq!(ranked[0].score, 0.9);
	assert_eq!(ranked[1].item, "mid");
	assert_eq!(ranked[1].score, 0.5);
	assert_eq!(ranked[2].item, "low");
	assert_eq!(ranked[2].score, 0.2);

	// verify original_rank is preserved
	assert_eq!(ranked[0].original_rank, 1);
	assert_eq!(ranked[1].original_rank, 2);
	assert_eq!(ranked[2].original_rank, 0);
}
