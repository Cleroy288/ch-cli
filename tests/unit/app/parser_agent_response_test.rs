//! Tests for app::parser_agent_response

use rustean::app::parser_agent_response::{
	agent_at_offset, parse_agent_sections,
};

#[test]
fn no_headers_returns_empty() {
	// Arrange
	let input = "Some plain text\nNo agents.";

	// Act
	let result = parse_agent_sections(input);

	// Assert
	assert!(result.is_empty());
}

#[test]
fn one_numbered_section() {
	// Arrange
	let input =
		"## 1. [sonnet] Review security\n\
		 Some code here";

	// Act
	let result = parse_agent_sections(input);

	// Assert
	assert_eq!(result.len(), 1);
	assert_eq!(result[0].model, "sonnet");
	assert_eq!(
		result[0].description,
		"Review security",
	);
	assert_eq!(result[0].start, 0);
	assert_eq!(result[0].end, input.len());
}

#[test]
fn two_sections_correct_ranges() {
	// Arrange
	let input =
		"## 1. [sonnet] First task\n\
		 block one\n\
		 ## 2. [opus] Second task\n\
		 block two";

	// Act
	let result = parse_agent_sections(input);

	// Assert
	assert_eq!(result.len(), 2);
	assert_eq!(result[0].model, "sonnet");
	assert_eq!(result[1].model, "opus");
	// First section ends where second starts
	assert_eq!(result[0].end, result[1].start);
	assert_eq!(result[1].end, input.len());
}

#[test]
fn agent_at_offset_finds_correct_section() {
	// Arrange
	let input =
		"## 1. [sonnet] First\n\
		 code\n\
		 ## 2. [opus] Second\n\
		 more code";
	let sections = parse_agent_sections(input);

	// Act & Assert — offset 0 is in first section
	let first = agent_at_offset(&sections, 0);
	assert_eq!(first.unwrap().model, "sonnet");

	// Offset in second section
	let second_start = sections[1].start;
	let second =
		agent_at_offset(&sections, second_start);
	assert_eq!(second.unwrap().model, "opus");
}

#[test]
fn h3_bracket_format() {
	// Arrange
	let input =
		"### [haiku] Write tests\ntest code";

	// Act
	let result = parse_agent_sections(input);

	// Assert
	assert_eq!(result.len(), 1);
	assert_eq!(result[0].model, "haiku");
	assert_eq!(
		result[0].description, "Write tests",
	);
}

#[test]
fn agent_prefix_format() {
	// Arrange
	let input =
		"## Agent 1: [sonnet] Analyze code\n\
		 analysis here";

	// Act
	let result = parse_agent_sections(input);

	// Assert
	assert_eq!(result.len(), 1);
	assert_eq!(result[0].model, "sonnet");
	assert_eq!(
		result[0].description, "Analyze code",
	);
}

#[test]
fn agent_at_offset_none_before_first() {
	// Arrange
	let input =
		"preamble text\n\
		 ## 1. [sonnet] Task\n\
		 code";
	let sections = parse_agent_sections(input);

	// Act — offset 0 is before first section
	let result = agent_at_offset(&sections, 0);

	// Assert
	assert!(result.is_none());
}

#[test]
fn empty_model_not_matched() {
	// Arrange
	let input = "## 1. [] description\ncode";

	// Act
	let result = parse_agent_sections(input);

	// Assert
	assert!(result.is_empty());
}

#[test]
fn empty_description_not_matched() {
	// Arrange
	let input = "## 1. [sonnet]\ncode";

	// Act
	let result = parse_agent_sections(input);

	// Assert
	assert!(result.is_empty());
}
