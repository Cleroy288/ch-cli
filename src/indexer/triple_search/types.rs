use crate::indexer::search::SearchHit;

#[derive(Debug, Clone, Default)]
pub struct TripleIndexStats {
	pub code_count: usize,
	pub doc_count: usize,
	pub notes_count: usize,
}

impl TripleIndexStats {
	pub fn total(&self) -> usize {
		self.code_count + self.doc_count + self.notes_count
	}
}

#[derive(Debug, Clone, Copy)]
pub struct TripleLimits {
	pub code: usize,
	pub doc: usize,
	pub notes: usize,
}

impl TripleLimits {
	pub fn uniform(limit: usize) -> Self {
		Self {
			code: limit,
			doc: limit,
			notes: limit,
		}
	}
}

#[derive(Debug, Default)]
pub struct TripleSearchResults {
	pub code_results: Vec<SearchHit>,
	pub doc_results: Vec<SearchHit>,
	pub notes_results: Vec<SearchHit>,
}
