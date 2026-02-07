//! Persistence functions for trigram index
//!
//! Functions to save and load the index from disk

use std::fs;
use std::io;
use std::path::Path;

use crate::indexer::trigram::conversion::{
	string_to_trigram, trigram_to_string,
};
use crate::indexer::trigram::types::{SerializableIndex, TrigramIndex};

impl TrigramIndex {
	/// Save index to disk as JSON
	pub fn save(&self, path: &Path) -> io::Result<()> {
		// convert to serializable format
		let converted = self
			.index
			.iter()
			.map(|(k, v)| (trigram_to_string(k), v.clone()));
		let serializable = SerializableIndex {
			index: converted.collect(),
			file_count: self.file_count,
		};

		let content = serde_json::to_string(&serializable)
			.map_err(|e| {
				io::Error::new(io::ErrorKind::InvalidData, e)
			})?;

		fs::write(path, content)
	}

	/// Load index from disk JSON file
	pub fn load(path: &Path) -> io::Result<Self> {
		let content = fs::read_to_string(path)?;

		let serializable: SerializableIndex =
			serde_json::from_str(&content).map_err(|e| {
				io::Error::new(io::ErrorKind::InvalidData, e)
			})?;

		// convert back from serializable format
		let index = serializable
			.index
			.into_iter()
			.filter_map(|(k, v)| {
				string_to_trigram(&k).map(|t| (t, v))
			})
			.collect();

		Ok(Self {
			index,
			file_count: serializable.file_count,
		})
	}
}
