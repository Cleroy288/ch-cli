//! Vector Store — Persistence and Query
//!
//! Search, persist, and read-only accessors for the HNSW
//! vector store using hnsw_rs. Uses bincode for fast
//! serialization (5-10x faster than JSON).

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

use hnsw_rs::prelude::*;

use crate::retrieval::hybrid::vector_store::{
	PointMeta, SearchResult, VectorStore,
};
use crate::retrieval::hybrid::vector_store_csls::{
	rerank_with_local_scaling,
};
use crate::retrieval::hybrid::vector_store_paths::{
	hnsw_dump_dir, mean_vector_path,
	sigma_values_path, HNSW_DUMP_BASENAME,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

/// HNSW construction parameters
const MAX_NB_CONNECTION: usize = 24;
/// Build quality parameter
const EF_CONSTRUCTION: usize = 200;
/// Max number of layers
const MAX_LAYER: usize = 16;

impl VectorStore {
	/// Search with Local Scaling reranking (public API)
	pub fn search(
		&self,
		query: &[f32],
		top_k: usize,
	) -> Vec<SearchResult> {
		let centered = match &self.mean_vector {
			Some(mean) => {
				super::mean_center::center_query(
					query, mean,
				)
			}
			None => query.to_vec(),
		};

		if self.sigma_values.is_empty() {
			return self.search_raw(&centered, top_k);
		}
		let raw = self.search_raw(&centered, top_k * 2);
		rerank_with_local_scaling(
			raw,
			&self.sigma_values,
			top_k,
		)
	}

	/// Raw HNSW search using hnsw_rs on-the-fly index
	pub fn search_raw(
		&self,
		query: &[f32],
		top_k: usize,
	) -> Vec<SearchResult> {
		if !self.indexed || self.points.is_empty() {
			return Vec::new();
		}

		let hnsw = self.build_hnsw_index();
		let ef_search = top_k.max(24);
		let neighbours =
			hnsw.search(query, top_k, ef_search);

		neighbours
			.into_iter()
			.filter_map(|nbr| {
				let idx = nbr.d_id;
				self.points.get(idx).map(|point| {
					SearchResult {
						point: PointMeta::from_point(
							point,
						),
						distance: nbr.distance,
					}
				})
			})
			.collect()
	}

	/// Get number of points
	pub fn len(&self) -> usize {
		self.points.len()
	}

	/// Check if empty
	pub fn is_empty(&self) -> bool {
		self.points.is_empty()
	}
}

impl VectorStore {
	/// Check if index is built
	pub fn is_indexed(&self) -> bool {
		self.indexed
	}

	/// Get vector dimension (from first point, 0 if empty)
	pub fn vector_dim(&self) -> usize {
		self.points
			.first()
			.map(|point| point.vector.len())
			.unwrap_or(0)
	}
}

impl VectorStore {
	/// Persist points, mean, sigma, and HNSW graph
	/// to disk using bincode format.
	pub fn persist(&self) -> RetrievalResult<()> {
		let path = match &self.store_path {
			Some(store_path) => store_path,
			None => return Ok(()),
		};

		write_bincode(path, &self.points)?;
		persist_mean(path, &self.mean_vector)?;
		persist_sigma(path, &self.sigma_values)?;
		self.persist_hnsw_dump(path)?;
		Ok(())
	}

	/// Build an ephemeral hnsw_rs index from points
	pub(crate) fn build_hnsw_index(
		&self,
	) -> Hnsw<'_, f32, DistCosine> {
		let nb_elem = self.points.len();
		let hnsw = Hnsw::<f32, DistCosine>::new(
			MAX_NB_CONNECTION,
			nb_elem,
			MAX_LAYER,
			EF_CONSTRUCTION,
			DistCosine {},
		);

		for (idx, point) in
			self.points.iter().enumerate()
		{
			hnsw.insert((&point.vector, idx));
		}

		hnsw
	}

	/// Dump the HNSW graph to disk via file_dump.
	fn persist_hnsw_dump(
		&self,
		vectors_path: &Path,
	) -> RetrievalResult<()> {
		if !self.indexed || self.points.is_empty() {
			return Ok(());
		}
		let dir = hnsw_dump_dir(vectors_path);
		std::fs::create_dir_all(&dir)?;

		let hnsw = self.build_hnsw_index();
		hnsw.file_dump(&dir, HNSW_DUMP_BASENAME)
			.map_err(|err| {
				RetrievalError::IoError(
					std::io::Error::other(
						format!(
							"HNSW dump: {}",
							err
						),
					),
				)
			})?;
		Ok(())
	}
}

/// Write any serializable value as bincode to a file
fn write_bincode<T: serde::Serialize + ?Sized>(
	path: &Path,
	value: &T,
) -> RetrievalResult<()> {
	let file = File::create(path)?;
	let writer = BufWriter::new(file);
	bincode::serialize_into(writer, value).map_err(
		|err| {
			RetrievalError::IoError(
				std::io::Error::other(err.to_string()),
			)
		},
	)
}

/// Persist mean vector as bincode if present
fn persist_mean(
	path: &Path,
	mean_vector: &Option<Vec<f32>>,
) -> RetrievalResult<()> {
	if let Some(ref mean) = mean_vector {
		write_bincode(&mean_vector_path(path), mean)?;
	}
	Ok(())
}

/// Persist sigma values as bincode. Skips if empty.
fn persist_sigma(
	path: &Path,
	sigma_values: &[f32],
) -> RetrievalResult<()> {
	if sigma_values.is_empty() {
		return Ok(());
	}
	write_bincode(&sigma_values_path(path), sigma_values)
}
