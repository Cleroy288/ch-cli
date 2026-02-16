//! Vector Store — Loading from Disk
//!
//! Loads vectors from bincode (fast) with JSON fallback
//! for backward compatibility. Also loads persisted mean
//! and sigma values when available.

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use crate::retrieval::hybrid::vector_store::{
	VectorPoint, VectorStore,
};
use crate::retrieval::hybrid::vector_store_paths::{
	legacy_json_sibling, mean_vector_path,
	sigma_values_path,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

impl VectorStore {
	/// Load from disk (bincode preferred, JSON fallback)
	pub fn load(
		path: &Path,
	) -> RetrievalResult<Self> {
		// Try bincode first, fall back to JSON
		let points = read_bincode::<Vec<VectorPoint>>(path)
			.or_else(|_| read_json::<Vec<VectorPoint>>(path))?;
		let next_id = points
			.iter()
			.map(|point| point.id)
			.max()
			.unwrap_or(0) + 1;
		let mean_vector = load_mean_vector(path);
		let sigma = load_sigma_values(path);

		let mut store = Self {
			points,
			indexed: false,
			store_path: Some(path.to_path_buf()),
			next_id,
			sigma_values: sigma,
			mean_vector,
		};

		store.build_index()?;
		Ok(store)
	}
}

/// Load mean vector: bincode first, JSON fallback.
/// Returns None if both are missing.
fn load_mean_vector(
	vectors_path: &Path,
) -> Option<Vec<f32>> {
	let bin = mean_vector_path(vectors_path);
	if let Ok(mean) = read_bincode::<Vec<f32>>(&bin) {
		return Some(mean);
	}
	let json =
		legacy_json_sibling(vectors_path, "mean");
	read_json::<Vec<f32>>(&json).ok()
}

/// Load sigma values: bincode first, JSON fallback.
/// Returns empty vec if both are missing.
fn load_sigma_values(
	vectors_path: &Path,
) -> Vec<f32> {
	let bin = sigma_values_path(vectors_path);
	if let Ok(sigma) = read_bincode::<Vec<f32>>(&bin) {
		return sigma;
	}
	let json =
		legacy_json_sibling(vectors_path, "sigma");
	read_json::<Vec<f32>>(&json).unwrap_or_default()
}

/// Read a value from a bincode file
fn read_bincode<T: serde::de::DeserializeOwned>(
	path: &Path,
) -> RetrievalResult<T> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);
	bincode::deserialize_from(reader).map_err(|err| {
		RetrievalError::IoError(
			std::io::Error::other(err.to_string()),
		)
	})
}

/// Read a value from a JSON file (legacy migration)
fn read_json<T: serde::de::DeserializeOwned>(
	path: &Path,
) -> RetrievalResult<T> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);
	serde_json::from_reader(reader).map_err(|err| {
		RetrievalError::IoError(
			std::io::Error::other(err.to_string()),
		)
	})
}
