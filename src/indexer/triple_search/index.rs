use std::path::{Path, PathBuf};

use crate::indexer::search::{SearchIndex, SearchResult};
use crate::indexer::symbols::{ContentType, Symbol};

use super::types::TripleIndexStats;

pub struct TripleSearchIndex {
	pub(crate) code_index: SearchIndex,
	pub(crate) doc_index: SearchIndex,
	pub(crate) notes_index: SearchIndex,
	pub(crate) base_path: Option<PathBuf>,
}

impl TripleSearchIndex {
	pub fn in_memory() -> SearchResult<Self> {
		Ok(Self {
			code_index: SearchIndex::in_memory()?,
			doc_index: SearchIndex::in_memory()?,
			notes_index: SearchIndex::in_memory()?,
			base_path: None,
		})
	}

	pub fn open_or_create(
		base_path: &Path,
	) -> SearchResult<Self> {
		Ok(Self {
			code_index: SearchIndex::open_or_create(
				base_path.join("code").join("tantivy"),
			)?,
			doc_index: SearchIndex::open_or_create(
				base_path.join("docs").join("tantivy"),
			)?,
			notes_index: SearchIndex::open_or_create(
				base_path.join("notes").join("tantivy"),
			)?,
			base_path: Some(base_path.to_path_buf()),
		})
	}

	pub fn index_symbols(
		&self,
		symbols: &[Symbol],
	) -> SearchResult<TripleIndexStats> {
		let (code, doc, notes) = route_by_content(symbols);

		let code_count = index_if_nonempty(
			&self.code_index, &code,
		)?;
		let doc_count = index_if_nonempty(
			&self.doc_index, &doc,
		)?;
		let notes_count = index_if_nonempty(
			&self.notes_index, &notes,
		)?;

		Ok(TripleIndexStats {
			code_count,
			doc_count,
			notes_count,
		})
	}

	pub fn path(&self) -> Option<&Path> {
		self.base_path.as_deref()
	}
}

#[allow(clippy::type_complexity)]
fn route_by_content(
	symbols: &[Symbol],
) -> (Vec<Symbol>, Vec<Symbol>, Vec<Symbol>) {
	let mut code = Vec::new();
	let mut doc = Vec::new();
	let mut notes = Vec::new();

	for sym in symbols {
		match ContentType::from_path(&sym.location.file) {
			ContentType::Code => code.push(sym.clone()),
			ContentType::Doc => doc.push(sym.clone()),
			ContentType::Notes => notes.push(sym.clone()),
		}
	}

	(code, doc, notes)
}

fn index_if_nonempty(
	idx: &SearchIndex,
	symbols: &[Symbol],
) -> SearchResult<usize> {
	if symbols.is_empty() {
		return Ok(0);
	}
	idx.index_symbols(symbols)
}
