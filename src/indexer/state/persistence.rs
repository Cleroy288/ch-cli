//! Persistence methods for saving and loading index state.

use std::fs;
use std::io;
use std::path::Path;

use crate::indexer::semantic::SymbolReference;

use super::INDEX_VERSION;
use super::types::IndexState;

impl IndexState {
	/// Load index state from disk
	pub fn load(root: &Path) -> io::Result<Self> {
		// state_file: path to the state JSON file
		let state_file = Self::state_file(root);

		// content: raw JSON content from the file
		let content = fs::read_to_string(&state_file)?;

		// state: deserialized IndexState from JSON
		let state: IndexState = serde_json::from_str(&content)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

		// Check version compatibility
		if state.version != INDEX_VERSION {
			return Err(io::Error::new(
				io::ErrorKind::InvalidData,
				format!(
					"Index version mismatch: expected {}, found {}",
					INDEX_VERSION, state.version
				),
			));
		}

		Ok(state)
	}

	/// Save index state to disk
	pub fn save(&self) -> io::Result<()> {
		// index_dir: directory where index files are stored
		let index_dir = Self::index_dir(&self.root);
		fs::create_dir_all(&index_dir)?;

		// state_file: path to the state JSON file
		let state_file = Self::state_file(&self.root);

		// content: serialized JSON content
		let content = serde_json::to_string_pretty(self)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

		fs::write(&state_file, content)?;

		Ok(())
	}

	/// Save references to disk (separate from state for performance)
	pub fn save_references(&self) -> io::Result<()> {
		// index_dir: directory where index files are stored
		let index_dir = Self::index_dir(&self.root);
		fs::create_dir_all(&index_dir)?;

		// refs_file: path to the references JSON file
		let refs_file = Self::refs_file(&self.root);

		// content: serialized JSON content for references
		let content = serde_json::to_string(&self.references)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

		fs::write(&refs_file, content)?;

		Ok(())
	}

	/// Load references from disk
	pub fn load_references(root: &Path) -> io::Result<Vec<SymbolReference>> {
		// refs_file: path to the references JSON file
		let refs_file = Self::refs_file(root);

		if !refs_file.exists() {
			return Ok(Vec::new());
		}

		// content: raw JSON content from the file
		let content = fs::read_to_string(&refs_file)?;

		// refs: deserialized references vector
		let refs: Vec<SymbolReference> = serde_json::from_str(&content)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

		Ok(refs)
	}

	/// Check if an index exists for the given root
	pub fn exists(root: &Path) -> bool {
		Self::state_file(root).exists()
	}
}
