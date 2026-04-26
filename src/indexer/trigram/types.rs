use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

pub type Trigram = [u8; 3];

#[derive(Debug, Clone, Default)]
pub struct TrigramIndex {
	#[doc(hidden)]
	pub index: HashMap<Trigram, HashSet<PathBuf>>,
	#[doc(hidden)]
	pub file_count: usize,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct SerializableIndex {
	pub(crate) index: HashMap<String, HashSet<PathBuf>>,
	pub(crate) file_count: usize,
}

#[derive(Debug, Clone)]
pub struct TrigramStats {
	pub trigram_count: usize,
	pub file_count: usize,
	pub avg_files_per_trigram: f64,
}
