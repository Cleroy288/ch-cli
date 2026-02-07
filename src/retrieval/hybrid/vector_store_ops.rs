//! Vector Store — Mutation Operations
//!
//! Insert, batch insert, index building, and clear operations
//! for the HNSW vector store.

use instant_distance::Builder;

use crate::retrieval::hybrid::vector_store::{
	VectorPoint, VectorStore,
};
use crate::retrieval::RetrievalResult;

impl VectorStore {
	/// Add a point to the store (index needs rebuild after)
	pub fn insert(&mut self, point: VectorPoint) {
		self.points.push(point);
		self.index = None; // invalidate index
	}

	/// Add multiple points
	pub fn insert_batch(&mut self, points: Vec<VectorPoint>) {
		self.points.extend(points);
		self.index = None;
	}

	/// Build the HNSW index
	pub fn build_index(&mut self) -> RetrievalResult<()> {
		if self.points.is_empty() {
			self.index = None;
			return Ok(());
		}

		// create values (IDs)
		let values: Vec<u64> =
			self.points.iter().map(|p| p.id).collect();

		// build index
		let hnsw =
			Builder::default().build(self.points.clone(), values);
		self.index = Some(hnsw);
		self.next_id = self.points.len() as u64;

		Ok(())
	}

	/// Clear all points and invalidate index
	pub fn clear(&mut self) {
		self.points.clear();
		self.index = None;
		self.next_id = 0;
	}

	/// Generate next unique ID
	pub fn next_id(&mut self) -> u64 {
		let id = self.next_id;
		self.next_id += 1;
		id
	}
}
