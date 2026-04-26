use std::collections::HashMap;
use std::path::PathBuf;
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::indexer::semantic::SymbolReference;
use crate::indexer::symbols::Symbol;

use super::INDEX_VERSION;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileState {
	pub path: PathBuf,
	pub mtime: u64,
	pub size: u64,
	pub symbol_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileChange {
	Added,
	Modified,
	Deleted,
	Unchanged,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexState {
	pub version: u32,
	pub root: PathBuf,
	pub last_updated: u64,
	pub files: HashMap<PathBuf, FileState>,
	#[serde(skip)]
	pub symbols: Vec<Symbol>,
	#[serde(skip)]
	pub references: Vec<SymbolReference>,
}

impl IndexState {
	pub fn new(root: PathBuf) -> Self {
		let canonical_root =
			root.canonicalize().unwrap_or(root);

		Self {
			version: INDEX_VERSION,
			root: canonical_root,
			last_updated: SystemTime::now()
				.duration_since(SystemTime::UNIX_EPOCH)
				.map(|dur| dur.as_secs())
				.unwrap_or(0),
			files: HashMap::new(),
			symbols: Vec::new(),
			references: Vec::new(),
		}
	}
}

#[derive(Debug, Default, Clone)]
pub struct ChangeSet {
	pub added: Vec<PathBuf>,
	pub modified: Vec<PathBuf>,
	pub deleted: Vec<PathBuf>,
	pub unchanged: Vec<PathBuf>,
}

impl ChangeSet {
	pub fn has_changes(&self) -> bool {
		!self.added.is_empty()
			|| !self.modified.is_empty()
			|| !self.deleted.is_empty()
	}

	pub fn files_to_index(&self) -> Vec<&PathBuf> {
		self.added.iter().chain(self.modified.iter()).collect()
	}

	pub fn total_changes(&self) -> usize {
		self.added.len() + self.modified.len() + self.deleted.len()
	}
}
