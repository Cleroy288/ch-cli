//! Tests for review::questions -- per-block
//! question prompt building and answer storage.

use rustean::domain::review::{
	BlockIdx, ReviewBlock,
};
use rustean::review::InlineBlocks;
use rustean::review::questions::question_prompt;

/// Helper: create a block with code
fn one_block() -> Vec<ReviewBlock> {
	vec![ReviewBlock::new(
		0,
		"rs".to_string(),
		"fn main() {}".to_string(),
		None,
	)]
}

#[test]
fn question_prompt_includes_code_and_question() {
	// Arrange
	let lang = "rs";
	let code = "let x = 1;";
	let question = "What does this do?";

	// Act
	let prompt =
		question_prompt(lang, code, question);

	// Assert
	assert!(prompt.contains("```rs"));
	assert!(prompt.contains("let x = 1;"));
	assert!(prompt.contains("What does this do?"));
}

#[test]
fn submit_question_returns_prompt() {
	// Arrange
	let mut ib = InlineBlocks::activate(
		one_block(), "resp".to_string(),
	);
	ib.block_mut(BlockIdx(0))
		.unwrap()
		.question = "explain".to_string();

	// Act
	let result =
		ib.submit_question(BlockIdx(0));

	// Assert
	assert!(result.is_some());
	assert!(result.unwrap().contains("explain"));
}

#[test]
fn submit_empty_question_returns_none() {
	// Arrange
	let mut ib = InlineBlocks::activate(
		one_block(), "resp".to_string(),
	);

	// Act
	let result =
		ib.submit_question(BlockIdx(0));

	// Assert
	assert!(result.is_none());
}

#[test]
fn set_answer_stores_text() {
	// Arrange
	let mut ib = InlineBlocks::activate(
		one_block(), "resp".to_string(),
	);

	// Act
	ib.set_answer(
		BlockIdx(0),
		"It declares x.".to_string(),
	);

	// Assert
	assert_eq!(
		ib.blocks()[0].answer.as_deref(),
		Some("It declares x."),
	);
}

#[test]
fn submit_invalid_index_returns_none() {
	// Arrange
	let mut ib = InlineBlocks::activate(
		one_block(), "resp".to_string(),
	);

	// Act
	let result =
		ib.submit_question(BlockIdx(99));

	// Assert
	assert!(result.is_none());
}
