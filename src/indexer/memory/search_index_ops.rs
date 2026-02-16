//! Index and search operations on MemorySearchIndex.

use crate::domain::errors::memory::MemoryResult;
use crate::domain::memory::Interaction;

use super::search_convert::interaction_to_doc;
use super::search_index::{
	MemorySearchIndex, map_tantivy,
};
use super::search_index_query::{
	build_parser, collect_hits, execute_search,
};
use super::types::MemoryHit;

/// Tantivy writer buffer size in bytes (15 MB)
const WRITER_BUFFER_BYTES: usize = 15_000_000;

impl MemorySearchIndex {
	/// Index a single interaction
	pub fn index_interaction(
		&self,
		interaction: &Interaction,
	) -> MemoryResult<()> {
		let doc = interaction_to_doc(
			&self.fields, interaction,
		);
		let mut writer = self
			.index
			.writer(WRITER_BUFFER_BYTES)
			.map_err(map_tantivy)?;
		writer
			.add_document(doc)
			.map_err(map_tantivy)?;
		writer.commit().map_err(map_tantivy)?;
		Ok(())
	}

	/// Search interactions by keyword
	pub fn search(
		&self,
		query: &str,
		limit: usize,
	) -> MemoryResult<Vec<MemoryHit>> {
		let reader = self.reader()?;
		let searcher = reader.searcher();
		let parser = build_parser(
			&self.index, &self.fields,
		);
		let top = execute_search(
			&parser, &searcher, query, limit,
		)?;
		collect_hits(
			&self.fields, &searcher, top,
		)
	}

	/// Total number of indexed interactions
	pub fn count(&self) -> MemoryResult<u64> {
		let reader = self.reader()?;
		Ok(reader.searcher().num_docs())
	}
}
