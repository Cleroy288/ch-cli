//! IndexManager configuration methods.

use super::builder::IndexManager;

impl IndexManager {
	/// Enable semantic analysis for name resolution
	/// Also enables reference extraction
	pub fn with_semantic_analysis(
		mut self,
	) -> Self {
		self.flags.semantic_analysis = true;
		self.flags.reference_extraction = true;
		self
	}

	/// Enable reference extraction from AST
	pub fn with_reference_extraction(
		mut self,
	) -> Self {
		self.flags.reference_extraction = true;
		self
	}

	/// Enable persistent storage
	/// (saves index to disk, incremental indexing)
	pub fn with_persistence(mut self) -> Self {
		self.flags.persistence = true;
		self
	}
}
