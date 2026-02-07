//! Tests for retrieval::daemon::server::doc_handlers

use std::path::PathBuf;

use ch_cli::indexer::SymbolKind;
use ch_cli::retrieval::daemon::server::doc_handlers::{
	doc_entry_to_response,
};
use ch_cli::retrieval::docgen::DocEntry;

/// Verify doc_entry_to_response maps all fields
#[test]
fn test_doc_entry_to_response() {
	let mut entry = DocEntry::new(
		"my_function".to_string(),
		SymbolKind::Function,
		PathBuf::from("/project/src/lib.rs"),
		42,
	);
	entry.user_comment =
		Some("user doc comment".to_string());
	entry.llm_doc =
		Some("generated doc".to_string());
	entry.signature = Some(
		"fn my_function(x: i32) -> bool".to_string(),
	);
	entry.links.depends_on =
		vec!["helper".to_string()];
	entry.links.depended_by =
		vec!["caller_a".to_string()];
	entry.links.external_deps =
		vec!["serde".to_string()];
	entry.mark_ready("generated doc".to_string());

	let resp = doc_entry_to_response(&entry);

	assert_eq!(resp.name, "my_function");
	assert_eq!(resp.kind, "fn");
	assert_eq!(resp.file_path, "/project/src/lib.rs");
	assert_eq!(resp.line, 42);
	assert_eq!(
		resp.user_comment,
		Some("user doc comment".to_string())
	);
	assert_eq!(
		resp.llm_doc,
		Some("generated doc".to_string())
	);
	assert_eq!(
		resp.signature,
		Some(
			"fn my_function(x: i32) -> bool"
				.to_string()
		)
	);
	assert_eq!(
		resp.depends_on,
		vec!["helper".to_string()]
	);
	assert_eq!(
		resp.depended_by,
		vec!["caller_a".to_string()]
	);
	assert_eq!(
		resp.external_deps,
		vec!["serde".to_string()]
	);
	assert_eq!(resp.status, "ready");
}
