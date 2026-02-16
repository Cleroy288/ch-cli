//! Tests for template classification logic.

use std::path::PathBuf;

use rustean::indexer::SymbolKind;
use rustean::retrieval::docgen::template_classify::{
	classify_entry, DocStrategy,
};
use rustean::retrieval::docgen::DocEntry;

/// Helper: create entry with given kind
fn entry_with_kind(kind: SymbolKind) -> DocEntry {
	DocEntry::new(
		"test_sym".into(), kind,
		PathBuf::from("src/lib.rs"), 1,
	)
}

#[test]
fn classify_entry_with_user_comment_returns_user() {
	let entry = entry_with_kind(SymbolKind::Function)
		.with_user_comment("My doc".into());
	assert_eq!(
		classify_entry(&entry),
		DocStrategy::UserComment,
	);
}

#[test]
fn classify_document_chunk_returns_skip() {
	let entry =
		entry_with_kind(SymbolKind::DocumentChunk);
	assert_eq!(
		classify_entry(&entry),
		DocStrategy::Skip,
	);
}

#[test]
fn classify_constant_returns_template() {
	let entry =
		entry_with_kind(SymbolKind::Constant);
	assert_eq!(
		classify_entry(&entry),
		DocStrategy::Template,
	);
}

#[test]
fn classify_function_returns_llm() {
	let entry =
		entry_with_kind(SymbolKind::Function);
	assert_eq!(
		classify_entry(&entry),
		DocStrategy::Llm,
	);
}

#[test]
fn classify_simple_struct_returns_template() {
	let mut entry =
		entry_with_kind(SymbolKind::Struct);
	entry.links.children =
		vec!["field_a".into(), "field_b".into()];
	assert_eq!(
		classify_entry(&entry),
		DocStrategy::Template,
	);
}

#[test]
fn classify_complex_struct_returns_llm() {
	let mut entry =
		entry_with_kind(SymbolKind::Struct);
	// 3 children = complex
	entry.links.children = vec![
		"a".into(), "b".into(), "c".into(),
	];
	assert_eq!(
		classify_entry(&entry),
		DocStrategy::Llm,
	);
}

#[test]
fn classify_empty_comment_not_usable() {
	let entry = entry_with_kind(SymbolKind::Function)
		.with_user_comment("   ".into());
	// whitespace-only comment is not usable
	assert_eq!(
		classify_entry(&entry),
		DocStrategy::Llm,
	);
}

#[test]
fn classify_trait_returns_llm() {
	let entry = entry_with_kind(SymbolKind::Trait);
	assert_eq!(
		classify_entry(&entry),
		DocStrategy::Llm,
	);
}

#[test]
fn classify_enum_variant_returns_template() {
	let entry =
		entry_with_kind(SymbolKind::EnumVariant);
	assert_eq!(
		classify_entry(&entry),
		DocStrategy::Template,
	);
}

#[test]
fn classify_many_deps_struct_returns_llm() {
	let mut entry =
		entry_with_kind(SymbolKind::Struct);
	// 4 deps = complex
	entry.links.depends_on = vec![
		"a".into(), "b".into(),
		"c".into(), "d".into(),
	];
	assert_eq!(
		classify_entry(&entry),
		DocStrategy::Llm,
	);
}
