//! Tests for composite symbol template generation.

use std::path::PathBuf;

use rustean::indexer::SymbolKind;
use rustean::retrieval::docgen::template_composites::{
	generate_composite_doc,
};
use rustean::retrieval::docgen::DocEntry;

#[test]
fn template_module_with_children() {
	let mut entry = DocEntry::new(
		"utils".into(), SymbolKind::Module,
		PathBuf::from("src/utils/mod.rs"), 1,
	);
	entry.links.children =
		vec!["helper_a".into(), "helper_b".into()];

	let doc = generate_composite_doc(&entry);
	assert!(doc.contains("Module"));
	assert!(doc.contains("`utils`"));
	assert!(doc.contains("helper_a"));
	assert!(doc.contains("helper_b"));
}

#[test]
fn template_struct_with_fields() {
	let mut entry = DocEntry::new(
		"Config".into(), SymbolKind::Struct,
		PathBuf::from("src/config.rs"), 1,
	)
	.with_signature("pub struct Config".into());
	entry.links.children =
		vec!["host".into(), "port".into()];

	let doc = generate_composite_doc(&entry);
	assert!(doc.contains("struct"));
	assert!(doc.contains("`Config`"));
	assert!(doc.contains("host"));
}

#[test]
fn template_enum_with_variants() {
	let mut entry = DocEntry::new(
		"Status".into(), SymbolKind::Enum,
		PathBuf::from("src/status.rs"), 1,
	)
	.with_signature("pub enum Status".into());
	entry.links.children =
		vec!["Active".into(), "Inactive".into()];

	let doc = generate_composite_doc(&entry);
	assert!(doc.contains("enum"));
	assert!(doc.contains("`Status`"));
	assert!(doc.contains("Active"));
}

#[test]
fn template_impl_with_methods() {
	let mut entry = DocEntry::new(
		"Config".into(), SymbolKind::Impl,
		PathBuf::from("src/config.rs"), 10,
	);
	entry.links.children =
		vec!["new".into(), "load".into()];

	let doc = generate_composite_doc(&entry);
	assert!(doc.contains("Implementation"));
	assert!(doc.contains("`Config`"));
	assert!(doc.contains("2 methods"));
}

#[test]
fn template_impl_single_method() {
	let mut entry = DocEntry::new(
		"App".into(), SymbolKind::Impl,
		PathBuf::from("src/app.rs"), 5,
	);
	entry.links.children = vec!["new".into()];

	let doc = generate_composite_doc(&entry);
	assert!(doc.contains("1 method"));
}

#[test]
fn template_module_empty_children() {
	let entry = DocEntry::new(
		"empty_mod".into(), SymbolKind::Module,
		PathBuf::from("src/empty.rs"), 1,
	);

	let doc = generate_composite_doc(&entry);
	assert!(doc.contains("(none)"));
}

#[test]
fn template_unsupported_returns_empty() {
	let entry = DocEntry::new(
		"foo".into(), SymbolKind::Function,
		PathBuf::from("src/lib.rs"), 1,
	);
	let doc = generate_composite_doc(&entry);
	assert!(doc.is_empty());
}
