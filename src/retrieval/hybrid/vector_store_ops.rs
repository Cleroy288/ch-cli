//! Vector Store — Mutation Operations
//!
//! Insert, batch insert, index building, and clear operations
//! for the HNSW vector store.

use crate::retrieval::hybrid::vector_store::{
	VectorPoint, VectorStore,
};
use crate::retrieval::RetrievalResult;

impl VectorStore {
	/// Add a point to the store (index needs rebuild after)
	pub fn insert(&mut self, point: VectorPoint) {
		self.points.push(point);
		self.indexed = false;
	}

	/// Add multiple points
	pub fn insert_batch(
		&mut self, points: Vec<VectorPoint>,
	) {
		self.points.extend(points);
		self.indexed = false;
	}

	/// Build the HNSW index and precompute LS sigma
	pub fn build_index(&mut self) -> RetrievalResult<()> {
		if self.points.is_empty() {
			self.indexed = false;
			self.sigma_values = Vec::new();
			return Ok(());
		}

		// Mean-center only if not already centered
		if self.mean_vector.is_none() {
			let mean = super::mean_center::compute_mean(
				&self.points,
			);
			super::mean_center::apply_centering(
				&mut self.points, &mean,
			);
			self.mean_vector = Some(mean);
		}

		self.indexed = true;
		self.next_id = self.points.len() as u64;

		// Reuse cached sigma if loaded from disk
		if self.sigma_values.is_empty() {
			self.sigma_values =
				super::vector_store_csls::compute_all_sigma(
					self,
				);
		}

		Ok(())
	}

	/// Clear all points and invalidate index
	pub fn clear(&mut self) {
		self.points.clear();
		self.indexed = false;
		self.next_id = 0;
		self.sigma_values.clear();
		self.mean_vector = None;
	}

	/// Generate next unique ID
	pub fn next_id(&mut self) -> u64 {
		let id = self.next_id;
		self.next_id += 1;
		id
	}
}
