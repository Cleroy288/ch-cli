use std::path::Path;

use crate::indexer::trigram::conversion::{
	string_to_trigram, trigram_to_string,
};
use crate::indexer::trigram::types::{SerializableIndex, TrigramIndex};

impl TrigramIndex {
	pub fn save(
		&self,
		path: &Path,
	) -> std::io::Result<()> {
		// convert to serializable format
		let converted = self
			.index
			.iter()
			.map(|(key, files)| {
				(trigram_to_string(key), files.clone())
			});
		let serializable = SerializableIndex {
			index: converted.collect(),
			file_count: self.file_count,
		};

		let content = serde_json::to_string(&serializable)
			.map_err(|err| {
				std::io::Error::new(
					std::io::ErrorKind::InvalidData,
					err,
				)
			})?;

		std::fs::write(path, content)
	}

	pub fn load(path: &Path) -> std::io::Result<Self> {
		let content = std::fs::read_to_string(path)?;

		let serializable: SerializableIndex =
			serde_json::from_str(&content).map_err(|err| {
				std::io::Error::new(
					std::io::ErrorKind::InvalidData,
					err,
				)
			})?;

		// convert back from serializable format
		let index = serializable
			.index
			.into_iter()
			.filter_map(|(key, files)| {
				string_to_trigram(&key)
					.map(|tri| (tri, files))
			})
			.collect();

		Ok(Self {
			index,
			file_count: serializable.file_count,
		})
	}
}
