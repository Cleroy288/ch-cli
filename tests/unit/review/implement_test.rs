//! Tests for review::implement -- step-by-step
//! execution flow.

use rustean::domain::review::{
	BlockIdx, ReviewBlock,
};
use rustean::review::InlineBlocks;

/// Helper: blocks with and without file_path
fn mixed_blocks() -> Vec<ReviewBlock> {
	vec![
		ReviewBlock::new(
			0, "rs".to_string(),
			"code_0".to_string(),
			Some("src/a.rs".to_string()),
		),
		ReviewBlock::new(
			1, "rs".to_string(),
			"code_1".to_string(),
			None, // no file path
		),
		ReviewBlock::new(
			2, "rs".to_string(),
			"code_2".to_string(),
			Some("src/b.rs".to_string()),
		),
	]
}

#[test]
fn start_implement_sets_first_writable() {
	// Arrange
	let mut ib = InlineBlocks::activate(
		mixed_blocks(), "r".to_string(),
	);

	// Act
	ib.start_implement();

	// Assert — first writable is index 0
	assert_eq!(ib.impl_step(), Some(BlockIdx(0)));
}

#[test]
fn confirm_step_returns_path_and_content() {
	// Arrange
	let mut ib = InlineBlocks::activate(
		mixed_blocks(), "r".to_string(),
	);
	ib.start_implement();

	// Act
	let result = ib.confirm_step();

	// Assert
	let (path, content) = result.unwrap();
	assert_eq!(path, "src/a.rs");
	assert_eq!(content, "code_0");
	// Advances to block 2 (skips 1, no path)
	assert_eq!(ib.impl_step(), Some(BlockIdx(2)));
}

#[test]
fn skip_step_advances_without_writing() {
	// Arrange
	let mut ib = InlineBlocks::activate(
		mixed_blocks(), "r".to_string(),
	);
	ib.start_implement();

	// Act
	ib.skip_step();

	// Assert — advanced to block 2
	assert_eq!(ib.impl_step(), Some(BlockIdx(2)));
}

#[test]
fn confirm_last_step_freezes() {
	// Arrange
	let mut ib = InlineBlocks::activate(
		mixed_blocks(), "r".to_string(),
	);
	ib.start_implement();
	ib.confirm_step(); // block 0 → block 2

	// Act
	ib.confirm_step(); // block 2 → done

	// Assert
	assert!(ib.impl_step().is_none());
	assert!(ib.is_frozen());
}

#[test]
fn cancel_implement_clears_step() {
	// Arrange
	let mut ib = InlineBlocks::activate(
		mixed_blocks(), "r".to_string(),
	);
	ib.start_implement();

	// Act
	ib.cancel_implement();

	// Assert
	assert!(ib.impl_step().is_none());
	assert!(!ib.is_frozen());
}

#[test]
fn writable_count_excludes_pathless() {
	// Arrange
	let ib = InlineBlocks::activate(
		mixed_blocks(), "r".to_string(),
	);

	// Act / Assert
	assert_eq!(ib.writable_count(), 2);
}

#[test]
fn start_on_frozen_does_nothing() {
	// Arrange
	let mut ib = InlineBlocks::activate(
		mixed_blocks(), "r".to_string(),
	);
	ib.freeze();

	// Act
	ib.start_implement();

	// Assert
	assert!(ib.impl_step().is_none());
}

#[test]
fn impl_step_display_counts_writable() {
	// Arrange
	let mut ib = InlineBlocks::activate(
		mixed_blocks(), "r".to_string(),
	);
	ib.start_implement();

	// Assert — step 1 of 2
	assert_eq!(ib.impl_step_display(), 1);

	ib.confirm_step();

	// Assert — step 2 of 2
	assert_eq!(ib.impl_step_display(), 2);
}
