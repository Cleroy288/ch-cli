//! Vector Store — Loading from Disk
//!
//! Deserialization and index rebuilding for persistent stores.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::retrieval::hybrid::vector_store::{
	VectorPoint, VectorStore,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

impl VectorStore {
	/// Load from disk
	pub fn load(path: &Path) -> RetrievalResult<Self> {
		let file = File::open(path)?;
		let reader = BufReader::new(file);
		let points: Vec<VectorPoint> =
			serde_json::from_reader(reader).map_err(|e| {
				RetrievalError::Io(std::io::Error::other(
					e.to_string(),
				))
			})?;

		let next_id = points
			.iter()
			.map(|p| p.id)
			.max()
			.unwrap_or(0)
			+ 1;

		let mut store = Self {
			index: None,
			points,
			store_path: Some(path.to_path_buf()),
			next_id,
		};

		store.build_index()?;
		Ok(store)
	}
}
