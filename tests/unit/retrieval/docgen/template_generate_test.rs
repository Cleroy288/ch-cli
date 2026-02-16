//! Tests for simple symbol template generation.

use std::path::PathBuf;

use rustean::indexer::SymbolKind;
use rustean::retrieval::docgen::template_generate::{
	generate_template_doc,
};
use rustean::retrieval::docgen::template_helpers::{
	extract_type_from_sig, extract_visibility,
	join_truncated,
};
use rustean::retrieval::docgen::DocEntry;

#[test]
fn template_constant_with_type() {
	let entry = DocEntry::new(
		"MAX_SIZE".into(), SymbolKind::Constant,
		PathBuf::from("src/lib.rs"), 1,
	)
	.with_signature(
		"pub const MAX_SIZE: usize = 100".into(),
	);

	let doc = generate_template_doc(&entry);
	assert!(doc.contains("const"));
	assert!(doc.contains("`MAX_SIZE`"));
	assert!(doc.contains("`usize"));
}

#[test]
fn template_static_variable() {
	let entry = DocEntry::new(
		"COUNTER".into(), SymbolKind::Static,
		PathBuf::from("src/lib.rs"), 5,
	)
	.with_signature("static COUNTER: AtomicU64".into());

	let doc = generate_template_doc(&entry);
	assert!(doc.contains("static"));
	assert!(doc.contains("`COUNTER`"));
}

#[test]
fn template_type_alias() {
	let entry = DocEntry::new(
		"Result".into(), SymbolKind::TypeAlias,
		PathBuf::from("src/lib.rs"), 10,
	)
	.with_signature(
		"pub type Result = std::io::Result<()>".into(),
	);

	let doc = generate_template_doc(&entry);
	assert!(doc.contains("Type alias"));
	assert!(doc.contains("`Result`"));
}

#[test]
fn template_enum_variant() {
	let mut entry = DocEntry::new(
		"Ready".into(), SymbolKind::EnumVariant,
		PathBuf::from("src/status.rs"), 3,
	);
	entry.links.parent = Some("Status".into());

	let doc = generate_template_doc(&entry);
	assert!(doc.contains("Variant"));
	assert!(doc.contains("`Ready`"));
	assert!(doc.contains("`Status`"));
}

#[test]
fn template_field_with_parent() {
	let mut entry = DocEntry::new(
		"name".into(), SymbolKind::Field,
		PathBuf::from("src/user.rs"), 7,
	)
	.with_signature("name: String".into());
	entry.links.parent = Some("User".into());

	let doc = generate_template_doc(&entry);
	assert!(doc.contains("Field"));
	assert!(doc.contains("`name: String`"));
	assert!(doc.contains("`User`"));
}

#[test]
fn template_unsupported_kind_returns_empty() {
	let entry = DocEntry::new(
		"foo".into(), SymbolKind::Function,
		PathBuf::from("src/lib.rs"), 1,
	);
	let doc = generate_template_doc(&entry);
	assert!(doc.is_empty());
}

// -- helper tests --

#[test]
fn extract_type_from_colon_sig() {
	assert_eq!(
		extract_type_from_sig("name: String"),
		"String",
	);
}

#[test]
fn extract_type_from_equals_sig() {
	let sig = "pub type Res = io::Result<()>";
	let result = extract_type_from_sig(sig);
	assert!(result.contains("io::Result"));
}

#[test]
fn extract_visibility_pub() {
	assert_eq!(
		extract_visibility("pub fn foo()"), "pub",
	);
}

#[test]
fn extract_visibility_pub_crate() {
	assert_eq!(
		extract_visibility("pub(crate) struct X"),
		"pub(crate)",
	);
}

#[test]
fn extract_visibility_private() {
	assert_eq!(extract_visibility("fn bar()"), "");
}

#[test]
fn join_truncated_within_limit() {
	let items = vec!["a".into(), "b".into()];
	assert_eq!(join_truncated(&items, 5), "a, b");
}

#[test]
fn join_truncated_over_limit() {
	let items: Vec<String> =
		(0..10).map(|i| format!("x{}", i)).collect();
	let result = join_truncated(&items, 3);
	assert!(result.contains("... and 7 more"));
}

#[test]
fn join_truncated_empty() {
	let items: Vec<String> = vec![];
	assert_eq!(join_truncated(&items, 5), "(none)");
}
