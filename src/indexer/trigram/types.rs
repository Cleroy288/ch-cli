//! Type definitions for trigram indexing
//!
//! Contains core types: Trigram, TrigramStats, TrigramIndex, SerializableIndex

use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

/// A trigram (3-byte sequence)
pub type Trigram = [u8; 3];

/// Trigram index for fast file filtering
#[derive(Debug, Clone, Default)]
pub struct TrigramIndex {
	/// Map from trigram to files containing it
	#[doc(hidden)]
	pub index: HashMap<Trigram, HashSet<PathBuf>>,
	/// Total number of indexed files
	#[doc(hidden)]
	pub file_count: usize,
}

/// Serializable representation of the trigram index
#[derive(Serialize, Deserialize)]
pub(crate) struct SerializableIndex {
	/// Map from trigram (hex string) to files
	pub(crate) index: HashMap<String, HashSet<PathBuf>>,
	/// Total number of indexed files
	pub(crate) file_count: usize,
}

/// Statistics about the trigram index
#[derive(Debug, Clone)]
pub struct TrigramStats {
	/// Number of unique trigrams
	pub trigram_count: usize,
	/// Number of indexed files
	pub file_count: usize,
	/// Average files per trigram
	pub avg_files_per_trigram: f64,
}
