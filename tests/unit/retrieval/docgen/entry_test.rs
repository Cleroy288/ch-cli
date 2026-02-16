//! Tests for retrieval::docgen::entry

use std::path::PathBuf;

use rustean::indexer::SymbolKind;
use rustean::retrieval::docgen::entry_types::DocStatus;
use rustean::retrieval::docgen::DocEntry;

#[test]
fn test_doc_entry_id_generation() {
	let path = PathBuf::from("src/main.rs");
	let e1 = DocEntry::new(
		"main".into(), SymbolKind::Function,
		path.clone(), 10,
	);
	let e2 = DocEntry::new(
		"main".into(), SymbolKind::Function,
		path.clone(), 10,
	);
	assert_eq!(e1.id, e2.id);

	let e3 = DocEntry::new(
		"main".into(), SymbolKind::Function, path, 20,
	);
	assert_ne!(e1.id, e3.id);
}

#[test]
fn test_doc_status_lifecycle() {
	let mut entry = DocEntry::new(
		"test".into(), SymbolKind::Function,
		PathBuf::from("test.rs"), 1,
	);
	assert_eq!(entry.status, DocStatus::Pending);
	assert!(!entry.is_ready());

	entry.mark_generating();
	assert_eq!(entry.status, DocStatus::Generating);

	entry.mark_ready("Generated doc".into());
	assert_eq!(entry.status, DocStatus::Ready);
	assert!(entry.is_ready());
	assert_eq!(entry.llm_doc, Some("Generated doc".into()));
}

#[test]
fn test_combined_doc() {
	let mut entry = DocEntry::new(
		"test".into(), SymbolKind::Function,
		PathBuf::from("test.rs"), 1,
	);
	entry.user_comment = Some("User doc".into());
	entry.llm_doc = Some("LLM doc".into());
	let combined = entry.combined_doc();
	assert!(combined.contains("User doc"));
	assert!(combined.contains("LLM doc"));
}

#[test]
fn test_doc_entry_builders() {
	let entry = DocEntry::new(
		"foo".into(), SymbolKind::Function,
		PathBuf::from("src/main.rs"), 5,
	)
	.with_user_comment("User doc".into())
	.with_signature("fn foo(x: i32)".into())
	.with_code_snippet("fn foo(x: i32) {}".into());

	assert_eq!(entry.user_comment, Some("User doc".into()));
	assert_eq!(entry.signature, Some("fn foo(x: i32)".into()));
	assert_eq!(entry.code_snippet, "fn foo(x: i32) {}");
}

#[test]
fn test_doc_entry_mark_failed() {
	let mut entry = DocEntry::new(
		"broken_fn".into(), SymbolKind::Function,
		PathBuf::from("src/lib.rs"), 1,
	);
	assert_eq!(entry.status, DocStatus::Pending);
	entry.mark_generating();
	assert_eq!(entry.status, DocStatus::Generating);
	entry.mark_failed();
	assert_eq!(entry.status, DocStatus::Failed);
	assert!(!entry.is_ready());
	assert!(entry.llm_doc.is_none());
}

#[test]
fn test_doc_entry_display() {
	let entry = DocEntry::new(
		"my_func".into(), SymbolKind::Function,
		PathBuf::from("src/lib.rs"), 25,
	);
	let display = format!("{}", entry);
	assert!(display.contains("fn"));
	assert!(display.contains("my_func"));
	assert!(display.contains("src/lib.rs"));
	assert!(display.contains("25"));
	assert!(display.contains("pending"));
}
