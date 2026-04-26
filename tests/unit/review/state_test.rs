//! Tests for review::state -- InlineBlocks
//! activate, freeze, and focus management.

use rustean::domain::review::{
	BlockIdx, ReviewBlock,
};
use rustean::review::{InlineBlocks, InlineFocus};

/// Helper: create N test blocks
fn test_blocks(n: usize) -> Vec<ReviewBlock> {
	(0..n)
		.map(|i| {
			ReviewBlock::new(
				i,
				"rs".to_string(),
				format!("code_{}", i),
				Some(format!("src/{}.rs", i)),
			)
		})
		.collect()
}

#[test]
fn activate_creates_blocks_with_editors() {
	// Arrange
	let blocks = test_blocks(3);

	// Act
	let ib = InlineBlocks::activate(
		blocks, "response".to_string(),
	);

	// Assert
	assert_eq!(ib.blocks().len(), 3);
	assert!(!ib.is_frozen());
	assert!(ib.focus().is_none());
}

#[test]
fn freeze_sets_frozen_and_clears_focus() {
	// Arrange
	let blocks = test_blocks(2);
	let mut ib = InlineBlocks::activate(
		blocks, "resp".to_string(),
	);
	ib.set_focus(
		InlineFocus::CodeBlock(BlockIdx(0)),
	);

	// Act
	ib.freeze();

	// Assert
	assert!(ib.is_frozen());
	assert!(ib.focus().is_none());
}

#[test]
fn set_focus_and_clear_focus() {
	// Arrange
	let blocks = test_blocks(2);
	let mut ib = InlineBlocks::activate(
		blocks, "resp".to_string(),
	);

	// Act
	ib.set_focus(
		InlineFocus::Question(BlockIdx(1)),
	);

	// Assert
	assert_eq!(
		ib.focus(),
		Some(InlineFocus::Question(BlockIdx(1))),
	);

	// Act
	ib.clear_focus();

	// Assert
	assert!(ib.focus().is_none());
}

#[test]
fn cached_sections_preserved() {
	// Arrange
	let resp = "Hello\n```rs\ncode\n```";

	// Act
	let ib = InlineBlocks::activate(
		vec![], resp.to_string(),
	);

	// Assert — 2 sections: text + code
	assert_eq!(ib.cached_sections().len(), 2);
}

#[test]
fn activate_empty_blocks() {
	// Arrange / Act
	let ib = InlineBlocks::activate(
		vec![], "".to_string(),
	);

	// Assert
	assert_eq!(ib.blocks().len(), 0);
	assert!(!ib.is_frozen());
}
