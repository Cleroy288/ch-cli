//! Tests for domain::review -- ReviewBlock struct.

use rustean::domain::review::{
	BlockIdx, ReviewBlock,
};

#[test]
fn new_block_has_matching_original_and_edited() {
	let block = ReviewBlock::new(
		0,
		"rust".to_string(),
		"let x = 1;".to_string(),
		None,
	);
	assert_eq!(block.original, block.edited);
}

#[test]
fn is_modified_false_when_unchanged() {
	let block = ReviewBlock::new(
		0,
		"rs".to_string(),
		"code".to_string(),
		None,
	);
	assert!(!block.is_modified());
}

#[test]
fn is_modified_true_when_edited() {
	let mut block = ReviewBlock::new(
		0,
		"rs".to_string(),
		"code".to_string(),
		None,
	);
	block.edited = "new code".to_string();
	assert!(block.is_modified());
}

#[test]
fn file_path_stored_correctly() {
	let block = ReviewBlock::new(
		2,
		"ts".to_string(),
		"const x = 1;".to_string(),
		Some("src/app.ts".to_string()),
	);
	assert_eq!(block.index, BlockIdx(2));
	assert_eq!(block.lang, "ts");
	assert_eq!(
		block.file_path,
		Some("src/app.ts".to_string()),
	);
}
