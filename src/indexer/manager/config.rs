//! IndexManager configuration methods.

use super::builder::IndexManager;

impl IndexManager {
	/// Also enables reference extraction
	pub fn with_semantic_analysis(
		mut self,
	) -> Self {
		self.flags.semantic_analysis = true;
		self.flags.reference_extraction = true;
		self
	}

	pub fn with_reference_extraction(
		mut self,
	) -> Self {
		self.flags.reference_extraction = true;
		self
	}

	/// Enable persistent storage
	pub fn with_persistence(
		mut self,
	) -> Self {
		self.flags.persistence = true;
		self
	}
}
