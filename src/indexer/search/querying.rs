//! Query operations: search, fuzzy search, and filtered search.

use tantivy::collector::TopDocs;
use tantivy::query::{
	BooleanQuery, BoostQuery, FuzzyTermQuery,
	Occur, Query, QueryParser, TermQuery,
};
use tantivy::schema::IndexRecordOption;
use tantivy::{TantivyDocument, Term};

use crate::indexer::search::conversion::doc_to_symbol;
use crate::indexer::search::error::SearchResult;
use crate::indexer::search::index_core::SearchIndex;
use crate::indexer::search::types::SearchHit;
use crate::indexer::symbols::SymbolKind;

impl SearchIndex {
	/// Search for symbols by name and content (full-text search)
	pub fn search(
		&self,
		query: &str,
		limit: usize,
	) -> SearchResult<Vec<SearchHit>> {
		let reader = self.reader()?; // index reader for searching
		let searcher = reader.searcher(); // searcher instance

		// Search in both symbol_name and content fields
		let query_parser = QueryParser::for_index(
			&self.index,
			vec![self.fields.symbol_name, self.fields.content],
		);
		let query = query_parser.parse_query(query)?; // parsed query

		let top_docs = searcher // search results
			.search(&query, &TopDocs::with_limit(limit))?;

		let mut results = Vec::new(); // results to return
		for (score, doc_address) in top_docs {
			let doc: TantivyDocument = searcher.doc(doc_address)?;
			if let Some(symbol) = doc_to_symbol(&self.fields, &doc) {
				results.push(SearchHit { symbol, score });
			}
		}

		Ok(results)
	}

	/// Search with document type boosting at query time
	/// Source code boosted higher than documentation/notes
	pub fn search_with_boost(
		&self,
		query: &str,
		limit: usize,
	) -> SearchResult<Vec<SearchHit>> {
		let reader = self.reader()?; // index reader for searching
		let searcher = reader.searcher(); // searcher instance

		// Parse base query for symbol_name and content fields
		let query_parser = QueryParser::for_index(
			&self.index,
			vec![self.fields.symbol_name, self.fields.content],
		);
		let base_query = query_parser.parse_query(query)?; // base query

		// Create boost query for SourceCode documents (highest priority)
		let source_term = Term::from_field_text(
			self.fields.document_type,
			"SourceCode",
		);
		let source_query = TermQuery::new(source_term, IndexRecordOption::Basic);

		// Combine: base query + boosted source code query
		// SourceCode docs get additional 50% boost
		let must_base = (
			Occur::Must,
			Box::new(base_query.box_clone()) as Box<dyn Query>
		);
		let inner_must: Vec<(Occur, Box<dyn Query>)> = vec![
			(Occur::Must, Box::new(base_query) as Box<dyn Query>),
			(Occur::Must, Box::new(source_query) as Box<dyn Query>),
		];
		let inner_bool = Box::new(BooleanQuery::new(inner_must));
		let boost = 0.5_f32;
		let should_boost = (
			Occur::Should,
			Box::new(BoostQuery::new(inner_bool, boost))
				as Box<dyn Query>
		);
		let boosted_query =
			BooleanQuery::new(vec![must_base, should_boost]);

		let collector = TopDocs::with_limit(limit);
		let top_docs = searcher.search(&boosted_query, &collector)?;

		let mut results = Vec::new(); // results to return
		for (score, doc_address) in top_docs {
			let doc: TantivyDocument = searcher.doc(doc_address)?;
			if let Some(symbol) = doc_to_symbol(&self.fields, &doc) {
				results.push(SearchHit { symbol, score });
			}
		}

		Ok(results)
	}

	/// Fuzzy search for symbols (handles typos)
	pub fn fuzzy_search(
		&self,
		term: &str,
		distance: u8,
		limit: usize,
	) -> SearchResult<Vec<SearchHit>> {
		let reader = self.reader()?; // index reader
		let searcher = reader.searcher(); // searcher instance

		let term = Term::from_field_text( // search term
			self.fields.symbol_name,
			term,
		);
		let query = FuzzyTermQuery::new(term, distance, true);

		let top_docs = searcher // search results
			.search(&query, &TopDocs::with_limit(limit))?;

		let mut results = Vec::new(); // results to return
		for (score, doc_address) in top_docs {
			let doc: TantivyDocument = searcher.doc(doc_address)?;
			if let Some(symbol) = doc_to_symbol(&self.fields, &doc) {
				results.push(SearchHit { symbol, score });
			}
		}

		Ok(results)
	}

	/// Search for symbols of a specific kind
	pub fn search_by_kind(
		&self,
		kind: SymbolKind,
		limit: usize,
	) -> SearchResult<Vec<SearchHit>> {
		let reader = self.reader()?; // index reader
		let searcher = reader.searcher(); // searcher instance

		let term = Term::from_field_text( // kind term
			self.fields.symbol_kind,
			&kind.to_string(),
		);
		let query = TermQuery::new(term, IndexRecordOption::Basic);

		let top_docs = searcher // search results
			.search(&query, &TopDocs::with_limit(limit))?;

		let mut results = Vec::new(); // results to return
		for (score, doc_address) in top_docs {
			let doc: TantivyDocument = searcher.doc(doc_address)?;
			if let Some(symbol) = doc_to_symbol(&self.fields, &doc) {
				results.push(SearchHit { symbol, score });
			}
		}

		Ok(results)
	}

	/// Get the total number of indexed documents
	pub fn num_docs(&self) -> SearchResult<u64> {
		let reader = self.reader()?; // index reader
		Ok(reader.searcher().num_docs())
	}
}
