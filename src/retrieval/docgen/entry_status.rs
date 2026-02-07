//! DocEntry status lifecycle methods.

use crate::retrieval::docgen::entry::DocEntry;
use crate::retrieval::docgen::entry_types::DocStatus;

/// Status and staleness methods.
impl DocEntry {
	/// Check if entry needs regeneration.
	pub fn is_stale(&self, current_mtime: u64) -> bool {
		self.source_mtime < current_mtime
	}

	/// Check if doc is ready.
	pub fn is_ready(&self) -> bool {
		self.status == DocStatus::Ready
	}

	/// Mark as generating.
	pub fn mark_generating(&mut self) {
		self.status = DocStatus::Generating;
	}

	/// Mark as ready with generated doc.
	pub fn mark_ready(&mut self, llm_doc: String) {
		self.llm_doc = Some(llm_doc);
		self.status = DocStatus::Ready;
	}

	/// Mark as failed.
	pub fn mark_failed(&mut self) {
		self.status = DocStatus::Failed;
	}
}
