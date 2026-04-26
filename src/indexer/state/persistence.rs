use std::path::Path;

use crate::indexer::semantic::SymbolReference;

use super::INDEX_VERSION;
use super::types::IndexState;

impl IndexState {
	pub fn load(root: &Path) -> std::io::Result<Self> {
		let state_file = Self::state_file(root);
		let content = std::fs::read_to_string(&state_file)?;

		let state: IndexState =
			serde_json::from_str(&content).map_err(|err| {
				std::io::Error::new(
					std::io::ErrorKind::InvalidData, err,
				)
			})?;

		if state.version != INDEX_VERSION {
			return Err(std::io::Error::new(
				std::io::ErrorKind::InvalidData,
				format!(
					"Index version mismatch: expected {}, found {}",
					INDEX_VERSION, state.version
				),
			));
		}

		Ok(state)
	}

	pub fn save(&self) -> std::io::Result<()> {
		let index_dir = Self::index_dir(&self.root);
		std::fs::create_dir_all(&index_dir)?;

		let state_file = Self::state_file(&self.root);
		let content =
			serde_json::to_string_pretty(self).map_err(
				|err| {
					std::io::Error::new(
						std::io::ErrorKind::InvalidData,
						err,
					)
				},
			)?;

		std::fs::write(&state_file, content)
	}

	pub fn save_references(&self) -> std::io::Result<()> {
		let refs_dir = Self::refs_dir(&self.root);
		std::fs::create_dir_all(&refs_dir)?;

		super::ref_persistence_io::save_grouped_refs(
			&refs_dir,
			&self.references,
			&self.root,
		)
	}

	pub fn load_references(
		root: &Path,
	) -> std::io::Result<Vec<SymbolReference>> {
		let refs_dir = Self::refs_dir(root);

		super::ref_persistence_io::migrate_if_needed(
			root, &refs_dir,
		)?;

		super::ref_persistence_io::load_all_refs(
			&refs_dir,
		)
	}

	pub fn exists(root: &Path) -> bool {
		Self::state_file(root).exists()
	}
}
