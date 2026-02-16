use rustean::retrieval::hybrid::fusion::RankedItem;

/// Test RankedItem holds data correctly
#[test]
fn test_ranked_item_creation() {
	let item = RankedItem {
		item: "a",
		rank: 1,
		score: 1.0,
	};

	assert_eq!(item.item, "a");
	assert_eq!(item.rank, 1);
	assert!((item.score - 1.0).abs() < 1e-6);
}

/// Test RankedItem clone preserves values
#[test]
fn test_ranked_item_clone() {
	let item = RankedItem {
		item: "a",
		rank: 3,
		score: 0.75,
	};
	let cloned = item.clone();

	assert_eq!(cloned.item, "a");
	assert_eq!(cloned.rank, 3);
	assert!((cloned.score - 0.75).abs() < 1e-6);
}
