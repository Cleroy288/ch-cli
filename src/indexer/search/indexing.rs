//! Indexing operations: adding, updating, and removing symbols from the index.

use std::path::{Path, PathBuf};

use crate::indexer::search::conversion::symbol_to_doc;
use crate::indexer::search::error::SearchResult;
use crate::indexer::search::index_core::SearchIndex;
use crate::indexer::symbols::Symbol;

/// Batch of symbols grouped by file path
type FileSymbolBatch = [(PathBuf, Vec<Symbol>)];

impl SearchIndex {
	/// Index a batch of symbols
	pub fn index_symbols(&self, symbols: &[Symbol]) -> SearchResult<usize> {
		let mut writer = self.writer(50_000_000)?; // 50MB heap
		let count = symbols.len(); // number of symbols to index

		for symbol in symbols {
			let doc = symbol_to_doc(&self.fields, symbol);
			writer.add_document(doc)?;
		}

		writer.commit()?;
		Ok(count)
	}

	/// Update symbols for a file: delete existing + add new
	pub fn update_file_symbols(
		&self,
		file_path: &Path,
		symbols: &[Symbol],
	) -> SearchResult<usize> {
		let mut writer = self.writer(50_000_000)?; // writer instance

		// Delete existing documents for this file
		let term = tantivy::Term::from_field_text(
			self.fields.file_path,
			&file_path.to_string_lossy(),
		);
		writer.delete_term(term);

		// Add new symbols
		let count = symbols.len(); // number of symbols to add
		for symbol in symbols {
			let doc = symbol_to_doc(&self.fields, symbol);
			writer.add_document(doc)?;
		}

		writer.commit()?;
		Ok(count)
	}

	/// Batch update symbols for multiple files (faster than calling update_file_symbols repeatedly)
	///
	/// This method:
	/// 1. Deletes symbols for all specified files to delete
	/// 2. Adds symbols for all file results
	/// 3. Commits once at the end
	pub fn batch_update(
		&self,
		files_to_delete: &[PathBuf],
		file_results: &FileSymbolBatch,
	) -> SearchResult<usize> {
		let mut writer = self.writer(50_000_000)?; // writer instance
		let mut total_count = 0; // total symbols added

		// Delete symbols for deleted/modified files
		for file_path in files_to_delete {
			let term = tantivy::Term::from_field_text(
				self.fields.file_path,
				&file_path.to_string_lossy(),
			);
			writer.delete_term(term);
		}

		// Add new symbols for all files
		for (file_path, symbols) in file_results {
			// Delete existing symbols for this file first
			let term = tantivy::Term::from_field_text(
				self.fields.file_path,
				&file_path.to_string_lossy(),
			);
			writer.delete_term(term);

			// Add new symbols
			for symbol in symbols {
				let doc = symbol_to_doc(&self.fields, symbol);
				writer.add_document(doc)?;
			}
			total_count += symbols.len();
		}

		// Single commit at the end
		writer.commit()?;
		Ok(total_count)
	}

	/// Delete all documents for a specific file (for incremental updates)
	pub fn delete_file(&self, file_path: &Path) -> SearchResult<()> {
		let mut writer = self.writer(50_000_000)?; // writer instance
		let term = tantivy::Term::from_field_text(
			self.fields.file_path,
			&file_path.to_string_lossy(),
		);
		writer.delete_term(term);
		writer.commit()?;
		Ok(())
	}

	/// Clear all documents from the index
	pub fn clear(&self) -> SearchResult<()> {
		let mut writer = self.writer(50_000_000)?; // writer instance
		writer.delete_all_documents()?;
		writer.commit()?;
		Ok(())
	}
}
