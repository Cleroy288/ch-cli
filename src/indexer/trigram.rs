//! Trigram Index for Fast Text Search
//!
//! Implements a trigram-based index for fast pre-filtering of files
//! before semantic search. Based on the approach used by:
//! - rust-analyzer (find usages)
//! - Zoekt (Sourcegraph code search)
//!
//! A trigram is a sequence of 3 consecutive characters.
//! The index maps each trigram to the set of files containing it.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// A trigram (3-byte sequence)
pub type Trigram = [u8; 3];

/// Convert trigram to string for serialization
fn trigram_to_string(t: &Trigram) -> String {
	format!("{:02x}{:02x}{:02x}", t[0], t[1], t[2])
}

/// Convert string back to trigram
fn string_to_trigram(s: &str) -> Option<Trigram> {
	if s.len() != 6 {
		return None;
	}
	let b0 = u8::from_str_radix(&s[0..2], 16).ok()?;
	let b1 = u8::from_str_radix(&s[2..4], 16).ok()?;
	let b2 = u8::from_str_radix(&s[4..6], 16).ok()?;
	Some([b0, b1, b2])
}

/// Trigram index for fast file filtering
#[derive(Debug, Clone, Default)]
pub struct TrigramIndex {
	/// Map from trigram to files containing it
	index: HashMap<Trigram, HashSet<PathBuf>>,
	/// Total number of indexed files
	file_count: usize,
}

/// Serializable representation of the trigram index
#[derive(Serialize, Deserialize)]
struct SerializableIndex {
	/// Map from trigram (hex string) to files
	index: HashMap<String, HashSet<PathBuf>>,
	/// Total number of indexed files
	file_count: usize,
}

impl TrigramIndex {
	/// Create a new empty trigram index
	pub fn new() -> Self {
		Self {
			index: HashMap::new(),
			file_count: 0,
		}
	}

	/// Index a file's content
	pub fn index_file(&mut self, path: &Path, content: &str) {
		let trigrams = Self::extract_trigrams(content);

		for trigram in trigrams {
			self.index
				.entry(trigram)
				.or_default()
				.insert(path.to_path_buf());
		}

		self.file_count += 1;
	}

	/// Index multiple files from disk
	pub fn index_files(&mut self, files: &[PathBuf]) -> io::Result<()> {
		for path in files {
			if let Ok(content) = fs::read_to_string(path) {
				self.index_file(path, &content);
			}
		}
		Ok(())
	}

	/// Find candidate files that might contain the identifier
	pub fn candidate_files(&self, identifier: &str) -> HashSet<PathBuf> {
		let trigrams = Self::extract_trigrams(identifier);

		if trigrams.is_empty() {
			// identifier too short, return all files
			return self.all_files();
		}

		// intersect all trigram matches
		let mut result: Option<HashSet<PathBuf>> = None;

		for trigram in trigrams {
			if let Some(files) = self.index.get(&trigram) {
				match &mut result {
					None => result = Some(files.clone()),
					Some(r) => *r = r.intersection(files).cloned().collect(),
				}
			} else {
				// trigram not found, no matches
				return HashSet::new();
			}
		}

		result.unwrap_or_default()
	}

	/// Get all indexed files
	pub fn all_files(&self) -> HashSet<PathBuf> {
		self.index.values().flatten().cloned().collect()
	}

	/// Extract trigrams from a string
	fn extract_trigrams(s: &str) -> Vec<Trigram> {
		let bytes = s.as_bytes();
		if bytes.len() < 3 {
			return Vec::new();
		}

		bytes
			.windows(3)
			.map(|w| [w[0], w[1], w[2]])
			.collect()
	}

	/// Get statistics about the index
	pub fn stats(&self) -> TrigramStats {
		TrigramStats {
			trigram_count: self.index.len(),
			file_count: self.file_count,
			avg_files_per_trigram: if self.index.is_empty() {
				0.0
			} else {
				self.index.values().map(|v| v.len()).sum::<usize>() as f64
					/ self.index.len() as f64
			},
		}
	}

	/// Remove a file from the index
	pub fn remove_file(&mut self, path: &Path) {
		for files in self.index.values_mut() {
			files.remove(path);
		}
		self.file_count = self.file_count.saturating_sub(1);
	}

	/// Save index to disk
	pub fn save(&self, path: &Path) -> io::Result<()> {
		// convert to serializable format
		let serializable = SerializableIndex {
			index: self
				.index
				.iter()
				.map(|(k, v)| (trigram_to_string(k), v.clone()))
				.collect(),
			file_count: self.file_count,
		};

		let content = serde_json::to_string(&serializable)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
		fs::write(path, content)
	}

	/// Load index from disk
	pub fn load(path: &Path) -> io::Result<Self> {
		let content = fs::read_to_string(path)?;
		let serializable: SerializableIndex = serde_json::from_str(&content)
			.map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

		// convert back from serializable format
		let index = serializable
			.index
			.into_iter()
			.filter_map(|(k, v)| string_to_trigram(&k).map(|t| (t, v)))
			.collect();

		Ok(Self {
			index,
			file_count: serializable.file_count,
		})
	}
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

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_extract_trigrams() {
		let trigrams = TrigramIndex::extract_trigrams("hello");
		assert_eq!(trigrams.len(), 3); // "hel", "ell", "llo"
		assert_eq!(trigrams[0], [b'h', b'e', b'l']);
		assert_eq!(trigrams[1], [b'e', b'l', b'l']);
		assert_eq!(trigrams[2], [b'l', b'l', b'o']);
	}

	#[test]
	fn test_extract_trigrams_short() {
		assert!(TrigramIndex::extract_trigrams("ab").is_empty());
		assert!(TrigramIndex::extract_trigrams("").is_empty());
	}

	#[test]
	fn test_index_and_search() {
		let mut index = TrigramIndex::new();

		index.index_file(Path::new("a.rs"), "fn hello_world() {}");
		index.index_file(Path::new("b.rs"), "fn goodbye() {}");
		index.index_file(Path::new("c.rs"), "struct Hello { world: i32 }");

		// search for "hello" - should find a.rs
		let candidates = index.candidate_files("hello");
		assert!(candidates.contains(&PathBuf::from("a.rs")));

		// search for "world" - should find a.rs and c.rs
		let candidates = index.candidate_files("world");
		assert!(candidates.contains(&PathBuf::from("a.rs")));
		assert!(candidates.contains(&PathBuf::from("c.rs")));

		// search for "xyz" - should find nothing
		let candidates = index.candidate_files("xyz123");
		assert!(candidates.is_empty());
	}

	#[test]
	fn test_stats() {
		let mut index = TrigramIndex::new();
		index.index_file(Path::new("a.rs"), "hello world");

		let stats = index.stats();
		assert_eq!(stats.file_count, 1);
		assert!(stats.trigram_count > 0);
	}
}
