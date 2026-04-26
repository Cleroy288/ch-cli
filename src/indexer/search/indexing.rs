use std::path::{Path, PathBuf};

use crate::indexer::search::conversion::symbol_to_doc;
use crate::indexer::search::error::SearchResult;
use crate::indexer::search::index_core::SearchIndex;
use crate::indexer::symbols::Symbol;

type FileSymbolBatch = [(PathBuf, Vec<Symbol>)];

/// 50 MB
const WRITER_HEAP: usize = 50_000_000;

impl SearchIndex {
	pub fn index_symbols(
		&self,
		symbols: &[Symbol],
	) -> SearchResult<usize> {
		let mut writer = self.writer(WRITER_HEAP)?;
		for symbol in symbols {
			let doc = symbol_to_doc(&self.fields, symbol);
			writer.add_document(doc)?;
		}
		writer.commit()?;
		Ok(symbols.len())
	}

	pub fn update_file_symbols(
		&self,
		file_path: &Path,
		symbols: &[Symbol],
	) -> SearchResult<usize> {
		let mut writer = self.writer(WRITER_HEAP)?;
		delete_by_path(&mut writer, self.fields.file_path, file_path);

		for symbol in symbols {
			let doc = symbol_to_doc(&self.fields, symbol);
			writer.add_document(doc)?;
		}
		writer.commit()?;
		Ok(symbols.len())
	}

	/// Single commit at the end for performance.
	pub fn batch_update(
		&self,
		files_to_delete: &[PathBuf],
		file_results: &FileSymbolBatch,
	) -> SearchResult<usize> {
		let mut writer = self.writer(WRITER_HEAP)?;
		let mut total = 0;

		for path in files_to_delete {
			delete_by_path(&mut writer, self.fields.file_path, path);
		}

		for (path, symbols) in file_results {
			delete_by_path(&mut writer, self.fields.file_path, path);
			for symbol in symbols {
				writer.add_document(
					symbol_to_doc(&self.fields, symbol),
				)?;
			}
			total += symbols.len();
		}

		writer.commit()?;
		Ok(total)
	}

	pub fn delete_file(
		&self,
		file_path: &Path,
	) -> SearchResult<()> {
		let mut writer = self.writer(WRITER_HEAP)?;
		delete_by_path(&mut writer, self.fields.file_path, file_path);
		writer.commit()?;
		Ok(())
	}

	pub fn clear(&self) -> SearchResult<()> {
		let mut writer = self.writer(WRITER_HEAP)?;
		writer.delete_all_documents()?;
		writer.commit()?;
		Ok(())
	}
}

fn delete_by_path(
	writer: &mut tantivy::IndexWriter,
	field: tantivy::schema::Field,
	path: &Path,
) {
	let term = tantivy::Term::from_field_text(
		field, &path.to_string_lossy(),
	);
	writer.delete_term(term);
}
