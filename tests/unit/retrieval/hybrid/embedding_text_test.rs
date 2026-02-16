use std::path::PathBuf;

use rustean::indexer::symbols::{SymbolKind, Visibility};
use rustean::retrieval::hybrid::embedding_text::{
	file_context, kind_to_label, split_identifier,
	symbol_to_embedding_text,
};
use rustean::indexer::{ByteSpan, CodeLocation, Symbol};

#[test]
fn kind_to_label_returns_nl_label() {
	assert_eq!(kind_to_label(&SymbolKind::Function), "function");
	assert_eq!(kind_to_label(&SymbolKind::Struct), "data structure");
	assert_eq!(kind_to_label(&SymbolKind::Trait), "trait interface");
	assert_eq!(kind_to_label(&SymbolKind::Enum), "enumeration");
}

#[test]
fn split_identifier_snake_case() {
	assert_eq!(split_identifier("embed_text"), "embed text");
	assert_eq!(split_identifier("build_index"), "build index");
	assert_eq!(split_identifier("a"), "a");
}

#[test]
fn split_identifier_camel_case() {
	assert_eq!(
		split_identifier("VectorStore"),
		"vector store"
	);
	assert_eq!(
		split_identifier("HybridSearch"),
		"hybrid search"
	);
}

#[test]
fn file_context_strips_src_and_extension() {
	let path = PathBuf::from("src/retrieval/hybrid/vector_store.rs");
	let ctx = file_context(&path);
	assert_eq!(ctx, "retrieval hybrid vector store");
}

#[test]
fn symbol_to_embedding_text_private_no_parent() {
	// Arrange — Symbol::new defaults to Private
	let mut symbol = Symbol::new(
		"embed_text".to_string(),
		SymbolKind::Function,
		CodeLocation::new(
			PathBuf::from("src/retrieval/hybrid/embedding.rs"),
			10, 1, ByteSpan::ZERO,
		),
	);
	symbol.doc_comment =
		Some("Generate embedding for text".to_string());
	symbol.signature = Some(
		"fn embed_text(text: &str) -> Vec<f32>".to_string(),
	);

	// Act
	let text = symbol_to_embedding_text(&symbol);

	// Assert — private prefix, no "of" clause
	assert!(text.starts_with("private function embed text in"));
	assert!(text.contains("retrieval hybrid embedding"));
	assert!(text.contains("Generate embedding for text"));
	assert!(text.contains("fn embed_text"));
	assert!(!text.contains(" of "));
}

#[test]
fn symbol_to_embedding_text_public_with_parent() {
	// Arrange
	let mut symbol = Symbol::new(
		"search_raw".to_string(),
		SymbolKind::Method,
		CodeLocation::new(
			PathBuf::from(
				"src/retrieval/hybrid/vector_store.rs",
			),
			42, 1, ByteSpan::ZERO,
		),
	);
	symbol.visibility = Visibility::Public;
	symbol.parent = Some("VectorStore".to_string());
	symbol.signature = Some(
		"pub fn search_raw(&self) -> Vec<SearchResult>"
			.to_string(),
	);

	// Act
	let text = symbol_to_embedding_text(&symbol);

	// Assert — public prefix + parent context
	assert!(text.starts_with("public method search raw of"));
	assert!(text.contains("of vector store in"));
	assert!(text.contains("retrieval hybrid vector store"));
	assert!(text.contains("pub fn search_raw"));
}
