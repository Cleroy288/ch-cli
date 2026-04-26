use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::{Index, Searcher};

use crate::domain::errors::memory::MemoryResult;

use super::search_extract::doc_to_hit;
use super::search_index::map_tantivy;
use super::search_schema::MemoryFields;
use super::types::MemoryHit;

/// Scored search result from Tantivy
pub(crate) type ScoredDoc =
	(f32, tantivy::DocAddress);

pub(crate) fn build_parser(
	index: &Index,
	fields: &MemoryFields,
) -> QueryParser {
	QueryParser::for_index(
		index,
		vec![
			fields.input_text,
			fields.response_text,
		],
	)
}

pub(crate) fn execute_search(
	parser: &QueryParser,
	searcher: &Searcher,
	query: &str,
	limit: usize,
) -> MemoryResult<Vec<ScoredDoc>> {
	let (parsed, _) =
		parser.parse_query_lenient(query);
	searcher
		.search(
			&parsed,
			&TopDocs::with_limit(limit),
		)
		.map_err(map_tantivy)
}

pub(crate) fn collect_hits(
	fields: &MemoryFields,
	searcher: &Searcher,
	top_docs: Vec<ScoredDoc>,
) -> MemoryResult<Vec<MemoryHit>> {
	let mut hits = Vec::new();
	for (score, addr) in top_docs {
		let doc = searcher
			.doc(addr)
			.map_err(map_tantivy)?;
		if let Some(hit) =
			doc_to_hit(fields, &doc, score)
		{
			hits.push(hit);
		}
	}
	Ok(hits)
}
