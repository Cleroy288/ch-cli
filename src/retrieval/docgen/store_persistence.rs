//! DocStore persistence and statistics.
//!
//! Provides load/save/exists operations and
//! stats/completion tracking for the doc store.

use std::io;
use std::path::Path;

use crate::indexer::state::INDEX_DIR_NAME;
use crate::retrieval::docgen::entry_types::DocStatus;
use crate::retrieval::docgen::store_core::{
	DocStore, DocStoreStats, DOCS_FILE_NAME,
};

/// Persistence operations (load, save, exists).
impl DocStore {
	/// Load store from disk.
	pub fn load(
		project_path: &Path,
	) -> io::Result<Self> {
		let docs_file = project_path
			.join(INDEX_DIR_NAME)
			.join(DOCS_FILE_NAME);

		if !docs_file.exists() {
			return Ok(Self::new(project_path));
		}

		let entries = load_entries(&docs_file)?;
		let complete = entries.values().all(
			|entry| entry.status == DocStatus::Ready,
		);

		Ok(Self {
			entries,
			project_path: project_path.to_path_buf(),
			generation_complete: complete,
		})
	}

	/// Save store to disk.
	pub fn save(&self) -> io::Result<()> {
		let index_dir =
			self.project_path.join(INDEX_DIR_NAME);
		std::fs::create_dir_all(&index_dir)?;

		let docs_file = self.docs_file_path();
		let content =
			serde_json::to_string_pretty(&self.entries)
				.map_err(|err| {
					io::Error::new(
						io::ErrorKind::InvalidData,
						err,
					)
				})?;

		std::fs::write(&docs_file, content)?;
		Ok(())
	}

	/// Check if store exists on disk.
	pub fn exists(project_path: &Path) -> bool {
		project_path
			.join(INDEX_DIR_NAME)
			.join(DOCS_FILE_NAME)
			.exists()
	}
}

/// Load entries map from a docs JSON file.
fn load_entries(
	docs_file: &Path,
) -> io::Result<
	std::collections::HashMap<
		String,
		crate::retrieval::docgen::DocEntry,
	>,
> {
	let content =
		std::fs::read_to_string(docs_file)?;
	serde_json::from_str(&content).map_err(|err| {
		io::Error::new(
			io::ErrorKind::InvalidData,
			err,
		)
	})
}

/// Completion status and statistics.
impl DocStore {
	/// Update completion status based on entries.
	pub(crate) fn update_completion_status(
		&mut self,
	) {
		self.generation_complete =
			!self.entries.is_empty()
				&& self.entries.values().all(|entry| {
					entry.status == DocStatus::Ready
						|| entry.status == DocStatus::Failed
				});
	}

	/// Get generation statistics.
	pub fn stats(&self) -> DocStoreStats {
		let total = self.entries.len();
		let ready = self.count_status(DocStatus::Ready);
		let pending =
			self.count_status(DocStatus::Pending);
		let generating =
			self.count_status(DocStatus::Generating);
		let failed =
			self.count_status(DocStatus::Failed);

		DocStoreStats {
			total,
			ready,
			pending,
			generating,
			failed,
			is_complete: self.generation_complete,
		}
	}

	/// Count entries with a given status.
	fn count_status(&self, status: DocStatus) -> usize {
		self.entries
			.values()
			.filter(|entry| entry.status == status)
			.count()
	}
}
