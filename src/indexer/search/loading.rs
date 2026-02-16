//! Bulk loading: retrieve all symbols from the Tantivy index.
//!
//! Used when incremental indexing detects 0 changes
//! but consumers still need the full symbol set.

use tantivy::query::AllQuery;
use tantivy::TantivyDocument;

use crate::indexer::search::conversion::doc_to_symbol;
use crate::indexer::search::error::SearchResult;
use crate::indexer::search::index_core::SearchIndex;
use crate::indexer::Symbol;

impl SearchIndex {
	/// Load every symbol stored in the Tantivy index.
	///
	/// Iterates all live documents, converts each back
	/// to a `Symbol` via `doc_to_symbol`. Returns an
	/// empty vec when the index is empty.
	pub fn load_all_symbols(
		&self,
	) -> SearchResult<Vec<Symbol>> {
		let reader = self.reader()?; // index reader
		let searcher = reader.searcher(); // searcher
		let count = searcher.num_docs() as usize;

		if count == 0 {
			return Ok(Vec::new());
		}

		// Collect all doc addresses via AllQuery
		let collector =
			tantivy::collector::TopDocs::with_limit(count);
		let top_docs =
			searcher.search(&AllQuery, &collector)?;

		let mut symbols = Vec::with_capacity(count);
		for (_score, addr) in top_docs {
			let doc: TantivyDocument =
				searcher.doc(addr)?;
			if let Some(sym) =
				doc_to_symbol(&self.fields, &doc)
			{
				symbols.push(sym);
			}
		}

		Ok(symbols)
	}
}
