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

/// A single boolean query clause
type QueryClause = (Occur, Box<dyn Query>);

/// A list of boolean query clauses
type QueryClauses = Vec<QueryClause>;

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
		let (query, _errs) = query_parser.parse_query_lenient(query);

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
		let reader = self.reader()?;
		let searcher = reader.searcher();

		let query_parser = QueryParser::for_index(
			&self.index,
			vec![self.fields.symbol_name, self.fields.content],
		);
		let (base, _errs) =
			query_parser.parse_query_lenient(query);

		let boosted = build_source_boost(
			&self.fields, base,
		);
		let top = searcher.search(
			&boosted, &TopDocs::with_limit(limit),
		)?;

		collect_hits(&self.fields, &searcher, top)
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

/// Build a BooleanQuery that boosts SourceCode documents
fn build_source_boost(
	fields: &crate::indexer::search::schema::SchemaFields,
	base_query: Box<dyn Query>,
) -> BooleanQuery {
	let source_term = Term::from_field_text(
		fields.document_type,
		"SourceCode",
	);
	let source_q = TermQuery::new(
		source_term, IndexRecordOption::Basic,
	);
	let must_base: QueryClause = (
		Occur::Must,
		Box::new(base_query.box_clone()),
	);
	let inner: QueryClauses = vec![
		(Occur::Must, base_query),
		(Occur::Must, Box::new(source_q)),
	];
	let boosted = BoostQuery::new(
		Box::new(BooleanQuery::new(inner)), 0.5_f32,
	);
	let should: QueryClause = (
		Occur::Should, Box::new(boosted),
	);
	BooleanQuery::new(vec![must_base, should])
}

/// Collect search hits from scored document addresses
fn collect_hits(
	fields: &crate::indexer::search::schema::SchemaFields,
	searcher: &tantivy::Searcher,
	top_docs: Vec<(f32, tantivy::DocAddress)>,
) -> SearchResult<Vec<SearchHit>> {
	let mut results = Vec::new();
	for (score, addr) in top_docs {
		let doc: TantivyDocument = searcher.doc(addr)?;
		if let Some(sym) = doc_to_symbol(fields, &doc) {
			results.push(SearchHit { symbol: sym, score });
		}
	}
	Ok(results)
}
