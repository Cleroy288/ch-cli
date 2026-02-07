use ch_cli::retrieval::hybrid::fusion::{
	fuse_results_default, fuse_results_weighted,
	rrf_score, RankedItem,
};

#[test]
fn test_rrf_score() {
	// rank 1 with k=60 should give 1/61
	assert!((rrf_score(1, 60.0) - 1.0 / 61.0).abs() < 1e-6);
	// rank 10 with k=60 should give 1/70
	assert!((rrf_score(10, 60.0) - 1.0 / 70.0).abs() < 1e-6);
}

#[test]
fn test_fuse_disjoint() {
	// two disjoint result sets
	let keyword = vec![
		RankedItem { item: "a", rank: 1, score: 1.0 },
		RankedItem { item: "b", rank: 2, score: 0.8 },
	];
	let semantic = vec![
		RankedItem { item: "c", rank: 1, score: 0.1 },
		RankedItem { item: "d", rank: 2, score: 0.2 },
	];

	let fused = fuse_results_default(keyword, semantic, |s| *s);

	assert_eq!(fused.len(), 4);
	// all items should have same RRF score (1/61 or 1/62)
}

#[test]
fn test_fuse_overlap() {
	// overlapping results - "a" appears in both
	let keyword = vec![
		RankedItem { item: "a", rank: 1, score: 1.0 },
		RankedItem { item: "b", rank: 2, score: 0.8 },
	];
	let semantic = vec![
		RankedItem { item: "a", rank: 2, score: 0.1 },
		RankedItem { item: "c", rank: 1, score: 0.05 },
	];

	let fused = fuse_results_default(keyword, semantic, |s| *s);

	assert_eq!(fused.len(), 3);

	// "a" should be first (appears in both)
	assert_eq!(fused[0].item, "a");
	assert!(fused[0].keyword_rank.is_some());
	assert!(fused[0].semantic_rank.is_some());

	// "a" RRF score = 1/61 + 1/62
	let expected_score = 1.0 / 61.0 + 1.0 / 62.0;
	assert!((fused[0].rrf_score - expected_score).abs() < 1e-6);
}

#[test]
fn test_fuse_weighted() {
	let keyword = vec![RankedItem { item: "a", rank: 1, score: 1.0 }];
	let semantic = vec![RankedItem { item: "b", rank: 1, score: 0.1 }];

	// weight semantic 2x more than keyword
	let fused = fuse_results_weighted(keyword, semantic, |s| *s, 60.0, 1.0, 2.0);

	assert_eq!(fused.len(), 2);
	// "b" should be first (2x weight)
	assert_eq!(fused[0].item, "b");
}
