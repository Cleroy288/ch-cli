use tantivy::query::{
	BooleanQuery, BoostQuery, Occur, Query, TermQuery,
};
use tantivy::schema::IndexRecordOption;
use tantivy::{TantivyDocument, Term};

use super::conversion_read::doc_to_symbol;
use super::error::SearchResult;
use super::schema::SchemaFields;
use super::types::SearchHit;

/// A single boolean query clause
type QueryClause = (Occur, Box<dyn Query>);

/// A list of boolean query clauses
type QueryClauses = Vec<QueryClause>;

///
/// Structure: MUST(base) + SHOULD(boost(base AND SourceCode))
/// so SourceCode results rank higher without excluding others.
pub fn build_source_boost(
	fields: &SchemaFields,
	base_query: Box<dyn Query>,
) -> BooleanQuery {
	let source_term = Term::from_field_text(
		fields.document_type, "SourceCode",
	);
	let source_q = TermQuery::new(
		source_term, IndexRecordOption::Basic,
	);
	let base_clone = base_query.box_clone();
	let inner: QueryClauses = vec![
		(Occur::Must, Box::new(base_clone)),
		(Occur::Must, Box::new(source_q)),
	];
	let boosted = BoostQuery::new(
		Box::new(BooleanQuery::new(inner)), 0.5_f32,
	);

	BooleanQuery::new(vec![
		(Occur::Must, base_query),
		(Occur::Should, Box::new(boosted)),
	])
}

pub fn collect_hits(
	fields: &SchemaFields,
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
