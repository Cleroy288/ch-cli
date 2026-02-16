//! Tests for retrieval::docgen::prompts

use rustean::indexer::SymbolKind;
use rustean::retrieval::docgen::prompts::{
	build_prompt, format_user_comment_section,
	get_prompt_for_kind, PromptInput,
};

#[test]
fn test_get_prompt_for_kind() {
	assert!(
		get_prompt_for_kind(SymbolKind::Function)
			.contains("function")
	);
	assert!(
		get_prompt_for_kind(SymbolKind::Struct)
			.contains("struct")
	);
	assert!(
		get_prompt_for_kind(SymbolKind::Trait)
			.contains("trait")
	);
}

#[test]
fn test_format_user_comment_section() {
	let section = format_user_comment_section(
		Some("This is a test function"),
	);
	assert!(section.contains("Existing documentation"));
	assert!(section.contains("This is a test function"));

	let empty = format_user_comment_section(None);
	assert!(empty.is_empty());
}

#[test]
fn test_build_prompt() {
	let prompt = build_prompt(PromptInput {
		kind: SymbolKind::Function,
		name: "process_data",
		signature: Some(
			"fn process_data(input: &str) -> Result<String>",
		),
		code_snippet:
			"fn process_data(input: &str) -> Result<String> { ... }",
		user_comment: Some("Processes input data"),
		parent: None,
	});

	assert!(prompt.contains("process_data"));
	assert!(prompt.contains("fn process_data"));
	assert!(prompt.contains("Existing documentation"));
}
