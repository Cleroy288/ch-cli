use tantivy::collector::TopDocs;
use tantivy::query::{FuzzyTermQuery, QueryParser, TermQuery};
use tantivy::schema::IndexRecordOption;
use tantivy::Term;

use super::error::SearchResult;
use super::index_core::SearchIndex;
use super::querying_helpers::{build_source_boost, collect_hits};
use super::types::SearchHit;
use crate::indexer::symbols::SymbolKind;

impl SearchIndex {
	pub fn search(
		&self,
		query: &str,
		limit: usize,
	) -> SearchResult<Vec<SearchHit>> {
		let reader = self.reader()?;
		let searcher = reader.searcher();

		let parser = QueryParser::for_index(
			&self.index,
			vec![self.fields.symbol_name, self.fields.content],
		);
		let (query, _) = parser.parse_query_lenient(query);
		let top = searcher
			.search(&query, &TopDocs::with_limit(limit))?;

		collect_hits(&self.fields, &searcher, top)
	}

	/// Search with document type boosting at query time
	pub fn search_with_boost(
		&self,
		query: &str,
		limit: usize,
	) -> SearchResult<Vec<SearchHit>> {
		let reader = self.reader()?;
		let searcher = reader.searcher();

		let parser = QueryParser::for_index(
			&self.index,
			vec![self.fields.symbol_name, self.fields.content],
		);
		let (base, _) = parser.parse_query_lenient(query);
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
		let reader = self.reader()?;
		let searcher = reader.searcher();

		let term = Term::from_field_text(
			self.fields.symbol_name, term,
		);
		let query =
			FuzzyTermQuery::new(term, distance, true);
		let top = searcher
			.search(&query, &TopDocs::with_limit(limit))?;

		collect_hits(&self.fields, &searcher, top)
	}

	pub fn search_by_kind(
		&self,
		kind: SymbolKind,
		limit: usize,
	) -> SearchResult<Vec<SearchHit>> {
		let reader = self.reader()?;
		let searcher = reader.searcher();

		let term = Term::from_field_text(
			self.fields.symbol_kind,
			&kind.to_string(),
		);
		let query = TermQuery::new(
			term, IndexRecordOption::Basic,
		);
		let top = searcher
			.search(&query, &TopDocs::with_limit(limit))?;

		collect_hits(&self.fields, &searcher, top)
	}

	pub fn num_docs(&self) -> SearchResult<u64> {
		Ok(self.reader()?.searcher().num_docs())
	}
}
