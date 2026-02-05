//! Triple Search Index
//!
//! Provides separate Tantivy indexes for Code, Doc, and Notes content types.
//! Enables parallel search across all three indexes without interference.

use std::path::{Path, PathBuf};

use crate::indexer::search::{SearchHit, SearchIndex, SearchResult};
use crate::indexer::symbols::{ContentType, Symbol};

/// Stats from triple indexing operation
#[derive(Debug, Clone, Default)]
pub struct TripleIndexStats {
	/// number of code symbols indexed
	pub code_count: usize,
	/// number of doc symbols indexed
	pub doc_count: usize,
	/// number of notes symbols indexed
	pub notes_count: usize,
}

impl TripleIndexStats {
	/// Get total symbols indexed
	pub fn total(&self) -> usize {
		self.code_count + self.doc_count + self.notes_count
	}
}

/// Results from triple search operation
#[derive(Debug, Default)]
pub struct TripleSearchResults {
	/// search results from code index
	pub code_results: Vec<SearchHit>,
	/// search results from doc index
	pub doc_results: Vec<SearchHit>,
	/// search results from notes index
	pub notes_results: Vec<SearchHit>,
}

/// Triple search index with separate code, doc, and notes indexes.
/// Enables parallel search without content type interference.
pub struct TripleSearchIndex {
	/// index for code symbols (SourceCode + Test)
	code_index: SearchIndex,
	/// index for documentation (doc/*.md)
	doc_index: SearchIndex,
	/// index for notes (notes/*.md)
	notes_index: SearchIndex,
	/// base path for persistence (if enabled)
	base_path: Option<PathBuf>,
}

impl TripleSearchIndex {
	/// Create new in-memory triple index
	pub fn in_memory() -> SearchResult<Self> {
		let code_index = SearchIndex::in_memory()?;
		let doc_index = SearchIndex::in_memory()?;
		let notes_index = SearchIndex::in_memory()?;

		Ok(Self {
			code_index,
			doc_index,
			notes_index,
			base_path: None,
		})
	}

	/// Create persistent triple index at given path
	pub fn open_or_create(base_path: &Path) -> SearchResult<Self> {
		let code_path = base_path.join("code").join("tantivy");
		let doc_path = base_path.join("docs").join("tantivy");
		let notes_path = base_path.join("notes").join("tantivy");

		let code_index = SearchIndex::open_or_create(&code_path)?;
		let doc_index = SearchIndex::open_or_create(&doc_path)?;
		let notes_index = SearchIndex::open_or_create(&notes_path)?;

		Ok(Self {
			code_index,
			doc_index,
			notes_index,
			base_path: Some(base_path.to_path_buf()),
		})
	}

	/// Index symbols, routing each to the appropriate index
	pub fn index_symbols(&self, symbols: &[Symbol]) -> SearchResult<TripleIndexStats> {
		let mut code_symbols = Vec::new(); // symbols for code index
		let mut doc_symbols = Vec::new(); // symbols for doc index
		let mut notes_symbols = Vec::new(); // symbols for notes index

		// Route symbols by content type
		for symbol in symbols {
			let content_type = ContentType::from_path(&symbol.location.file);
			match content_type {
				ContentType::Code => code_symbols.push(symbol.clone()),
				ContentType::Doc => doc_symbols.push(symbol.clone()),
				ContentType::Notes => notes_symbols.push(symbol.clone()),
			}
		}

		// Index each set
		let code_count = if !code_symbols.is_empty() {
			self.code_index.index_symbols(&code_symbols)?
		} else {
			0
		};

		let doc_count = if !doc_symbols.is_empty() {
			self.doc_index.index_symbols(&doc_symbols)?
		} else {
			0
		};

		let notes_count = if !notes_symbols.is_empty() {
			self.notes_index.index_symbols(&notes_symbols)?
		} else {
			0
		};

		Ok(TripleIndexStats {
			code_count,
			doc_count,
			notes_count,
		})
	}

	/// Search code index only
	pub fn search_code(&self, query: &str, limit: usize) -> SearchResult<Vec<SearchHit>> {
		self.code_index.search(query, limit)
	}

	/// Search doc index only
	pub fn search_docs(&self, query: &str, limit: usize) -> SearchResult<Vec<SearchHit>> {
		self.doc_index.search(query, limit)
	}

	/// Search notes index only
	pub fn search_notes(&self, query: &str, limit: usize) -> SearchResult<Vec<SearchHit>> {
		self.notes_index.search(query, limit)
	}

	/// Parallel search all three indexes using rayon
	pub fn search_parallel(
		&self,
		query: &str,
		code_limit: usize,
		doc_limit: usize,
		notes_limit: usize,
	) -> SearchResult<TripleSearchResults> {
		// Use rayon::join for parallel execution
		let ((code_result, doc_result), notes_result) = rayon::join(
			|| {
				rayon::join(
					|| self.search_code(query, code_limit),
					|| self.search_docs(query, doc_limit),
				)
			},
			|| self.search_notes(query, notes_limit),
		);

		// Propagate any errors
		let code_results = code_result?;
		let doc_results = doc_result?;
		let notes_results = notes_result?;

		Ok(TripleSearchResults {
			code_results,
			doc_results,
			notes_results,
		})
	}

	/// Sequential search all three indexes (fallback if rayon unavailable)
	pub fn search_sequential(
		&self,
		query: &str,
		code_limit: usize,
		doc_limit: usize,
		notes_limit: usize,
	) -> SearchResult<TripleSearchResults> {
		let code_results = self.search_code(query, code_limit)?;
		let doc_results = self.search_docs(query, doc_limit)?;
		let notes_results = self.search_notes(query, notes_limit)?;

		Ok(TripleSearchResults {
			code_results,
			doc_results,
			notes_results,
		})
	}

	/// Get base path (if persistent)
	pub fn path(&self) -> Option<&Path> {
		self.base_path.as_deref()
	}

	/// Get total symbol count across all indexes
	pub fn total_count(&self) -> usize {
		// Note: This requires reader access which may not be available
		// For now, return 0 as a placeholder
		0
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::indexer::symbols::{CodeLocation, SymbolKind};

	/// Create a test symbol at given path
	fn create_test_symbol(name: &str, path: &str) -> Symbol {
		Symbol::new(
			name.to_string(),
			SymbolKind::Function,
			CodeLocation::new(PathBuf::from(path), 1, 0, 0, 10),
		)
	}

	/// Test TripleSearchIndex routes symbols correctly
	#[test]
	fn test_triple_index_routes_symbols() {
		let index = TripleSearchIndex::in_memory().unwrap();

		// Use paths with leading / to match DocumentType patterns
		let symbols = vec![
			create_test_symbol("code_func", "/project/src/main.rs"),
			create_test_symbol("test_func", "/project/tests/test.rs"),
			create_test_symbol("doc_section", "/project/doc/guide.md"),
			create_test_symbol("note_section", "/project/notes/impl.md"),
			create_test_symbol("bench_section", "/project/notes/benchmarks/perf.md"),
		];

		let stats = index.index_symbols(&symbols).unwrap();

		// Code: src/main.rs + tests/test.rs = 2
		assert_eq!(stats.code_count, 2);
		// Doc: doc/guide.md = 1
		assert_eq!(stats.doc_count, 1);
		// Notes: notes/impl.md + notes/benchmarks/perf.md = 2
		assert_eq!(stats.notes_count, 2);
		// Total: 5
		assert_eq!(stats.total(), 5);
	}

	/// Test TripleSearchIndex searches code index only
	#[test]
	fn test_triple_index_search_code() {
		let index = TripleSearchIndex::in_memory().unwrap();

		let symbols = vec![
			create_test_symbol("handle_request", "/project/src/handler.rs"),
			create_test_symbol("handle_docs", "/project/doc/api.md"),
		];

		index.index_symbols(&symbols).unwrap();

		// Search code index - should find handle_request
		let results = index.search_code("handle", 10).unwrap();
		assert_eq!(results.len(), 1);
		assert_eq!(results[0].symbol.name, "handle_request");
	}

	/// Test TripleSearchIndex searches doc index only
	#[test]
	fn test_triple_index_search_docs() {
		let index = TripleSearchIndex::in_memory().unwrap();

		let symbols = vec![
			create_test_symbol("handle_request", "/project/src/handler.rs"),
			create_test_symbol("handle_docs", "/project/doc/api.md"),
		];

		index.index_symbols(&symbols).unwrap();

		// Search doc index - should find handle_docs
		let results = index.search_docs("handle", 10).unwrap();
		assert_eq!(results.len(), 1);
		assert_eq!(results[0].symbol.name, "handle_docs");
	}

	/// Test TripleSearchIndex searches notes index only
	#[test]
	fn test_triple_index_search_notes() {
		let index = TripleSearchIndex::in_memory().unwrap();

		let symbols = vec![
			create_test_symbol("handle_request", "/project/src/handler.rs"),
			create_test_symbol("handle_notes", "/project/notes/impl.md"),
		];

		index.index_symbols(&symbols).unwrap();

		// Search notes index - should find handle_notes
		let results = index.search_notes("handle", 10).unwrap();
		assert_eq!(results.len(), 1);
		assert_eq!(results[0].symbol.name, "handle_notes");
	}

	/// Test TripleSearchIndex parallel search returns all results
	#[test]
	fn test_triple_index_parallel_search() {
		let index = TripleSearchIndex::in_memory().unwrap();

		let symbols = vec![
			create_test_symbol("handle_request", "/project/src/handler.rs"),
			create_test_symbol("handle_docs", "/project/doc/api.md"),
			create_test_symbol("handle_notes", "/project/notes/impl.md"),
		];

		index.index_symbols(&symbols).unwrap();

		// Parallel search all indexes
		let results = index.search_parallel("handle", 10, 10, 10).unwrap();

		assert_eq!(results.code_results.len(), 1);
		assert_eq!(results.doc_results.len(), 1);
		assert_eq!(results.notes_results.len(), 1);

		assert_eq!(results.code_results[0].symbol.name, "handle_request");
		assert_eq!(results.doc_results[0].symbol.name, "handle_docs");
		assert_eq!(results.notes_results[0].symbol.name, "handle_notes");
	}

	/// Test TripleSearchIndex with empty symbols
	#[test]
	fn test_triple_index_empty() {
		let index = TripleSearchIndex::in_memory().unwrap();

		let stats = index.index_symbols(&[]).unwrap();

		assert_eq!(stats.code_count, 0);
		assert_eq!(stats.doc_count, 0);
		assert_eq!(stats.notes_count, 0);
		assert_eq!(stats.total(), 0);
	}

	/// Test TripleSearchIndex respects limits
	#[test]
	fn test_triple_index_respects_limits() {
		let index = TripleSearchIndex::in_memory().unwrap();

		// Use names that share a common word for Tantivy tokenization
		let symbols = vec![
			create_test_symbol("handle_alpha", "/project/src/a.rs"),
			create_test_symbol("handle_beta", "/project/src/b.rs"),
			create_test_symbol("handle_gamma", "/project/src/c.rs"),
		];

		index.index_symbols(&symbols).unwrap();

		// Search with limit of 1
		let results = index.search_code("handle", 1).unwrap();
		assert_eq!(results.len(), 1);

		// Search with limit of 2
		let results = index.search_code("handle", 2).unwrap();
		assert_eq!(results.len(), 2);
	}
}
