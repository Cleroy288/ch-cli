//! Index state management for incremental parsing and persistence.
//!
//! This module provides:
//! - FileState: Track individual file metadata (mtime, hash)
//! - IndexState: Manage the overall index state
//! - Persistence: Save/load index state to disk

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use serde::{Deserialize, Serialize};

use crate::indexer::semantic::SymbolReference;
use crate::indexer::symbols::Symbol;

/// The directory name for storing index data
pub const INDEX_DIR_NAME: &str = ".ch-index";

/// State of a single indexed file
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileState {
    /// Path to the file (relative to project root)
    pub path: PathBuf,
    /// Last modification time (as seconds since UNIX epoch)
    pub mtime: u64,
    /// File size in bytes
    pub size: u64,
    /// Number of symbols extracted from this file
    pub symbol_count: usize,
}

impl FileState {
    /// Create a new FileState from a file path
    pub fn from_path(path: &Path, root: &Path) -> io::Result<Self> {
        let metadata = fs::metadata(path)?;
        let mtime = metadata
            .modified()?
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let relative_path = path.strip_prefix(root).unwrap_or(path).to_path_buf();

        Ok(Self {
            path: relative_path,
            mtime,
            size: metadata.len(),
            symbol_count: 0,
        })
    }

    /// Check if the file has changed compared to current disk state
    pub fn has_changed(&self, root: &Path) -> bool {
        let full_path = root.join(&self.path);
        match fs::metadata(&full_path) {
            Ok(metadata) => {
                let current_mtime = metadata
                    .modified()
                    .ok()
                    .and_then(|t| t.duration_since(SystemTime::UNIX_EPOCH).ok())
                    .map(|d| d.as_secs())
                    .unwrap_or(0);
                let current_size = metadata.len();

                current_mtime != self.mtime || current_size != self.size
            }
            Err(_) => true, // File doesn't exist or can't be read
        }
    }
}

/// Change detection result for a file
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FileChange {
    /// File is new (not in previous index)
    Added,
    /// File has been modified
    Modified,
    /// File has been deleted
    Deleted,
    /// File is unchanged
    Unchanged,
}

/// The complete index state for a project
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexState {
    /// Version of the index format
    pub version: u32,
    /// Root directory of the indexed project
    pub root: PathBuf,
    /// When the index was last updated (seconds since UNIX epoch)
    pub last_updated: u64,
    /// State of each indexed file
    pub files: HashMap<PathBuf, FileState>,
    /// All symbols from all files
    #[serde(skip)]
    pub symbols: Vec<Symbol>,
    /// All references from all files (for cross-file usages)
    #[serde(skip)]
    pub references: Vec<SymbolReference>,
}

/// Current index format version
pub const INDEX_VERSION: u32 = 1;

impl IndexState {
    /// Create a new empty index state
    pub fn new(root: PathBuf) -> Self {
        // Canonicalize the root path to avoid path comparison issues
        let root = root.canonicalize().unwrap_or(root);

        Self {
            version: INDEX_VERSION,
            root,
            last_updated: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0),
            files: HashMap::new(),
            symbols: Vec::new(),
            references: Vec::new(),
        }
    }

    /// Get the index directory path
    pub fn index_dir(root: &Path) -> PathBuf {
        root.join(INDEX_DIR_NAME)
    }

    /// Get the state file path
    pub fn state_file(root: &Path) -> PathBuf {
        Self::index_dir(root).join("state.json")
    }

    /// Get the Tantivy index directory
    pub fn tantivy_dir(root: &Path) -> PathBuf {
        Self::index_dir(root).join("tantivy")
    }

    /// Get the references cache file path
    pub fn refs_file(root: &Path) -> PathBuf {
        Self::index_dir(root).join("refs.json")
    }

    /// Get the trigram index file path
    pub fn trigram_file(root: &Path) -> PathBuf {
        Self::index_dir(root).join("trigrams.json")
    }

    /// Load index state from disk
    pub fn load(root: &Path) -> io::Result<Self> {
        let state_file = Self::state_file(root);
        let content = fs::read_to_string(&state_file)?;
        let state: IndexState =
            serde_json::from_str(&content).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

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
        let index_dir = Self::index_dir(&self.root);
        fs::create_dir_all(&index_dir)?;

        let state_file = Self::state_file(&self.root);
        let content = serde_json::to_string_pretty(self)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&state_file, content)?;

        Ok(())
    }

    /// Save references to disk (separate from state for performance)
    pub fn save_references(&self) -> io::Result<()> {
        let index_dir = Self::index_dir(&self.root);
        fs::create_dir_all(&index_dir)?;

        let refs_file = Self::refs_file(&self.root);
        let content = serde_json::to_string(&self.references)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        fs::write(&refs_file, content)?;

        Ok(())
    }

    /// Load references from disk
    pub fn load_references(root: &Path) -> io::Result<Vec<SymbolReference>> {
        let refs_file = Self::refs_file(root);
        if !refs_file.exists() {
            return Ok(Vec::new());
        }

        let content = fs::read_to_string(&refs_file)?;
        let refs: Vec<SymbolReference> = serde_json::from_str(&content)
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

        Ok(refs)
    }

    /// Check if an index exists for the given root
    pub fn exists(root: &Path) -> bool {
        Self::state_file(root).exists()
    }

    /// Update file state after indexing
    pub fn update_file(&mut self, path: &Path, symbol_count: usize) -> io::Result<()> {
        // Canonicalize the path to match the canonicalized root
        let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let mut file_state = FileState::from_path(&canonical_path, &self.root)?;
        file_state.symbol_count = symbol_count;
        self.files.insert(file_state.path.clone(), file_state);
        Ok(())
    }

    /// Remove a file from the index
    pub fn remove_file(&mut self, path: &Path) {
        // Canonicalize the path to match stored paths
        let canonical_path = path.canonicalize().unwrap_or_else(|_| path.to_path_buf());
        let relative = canonical_path.strip_prefix(&self.root).unwrap_or(&canonical_path);
        self.files.remove(relative);
    }

    /// Detect changes between current disk state and indexed state
    pub fn detect_changes(&self, current_files: &[PathBuf]) -> ChangeSet {
        let mut changes = ChangeSet::default();

        // Check for added and modified files
        for file_path in current_files {
            // Canonicalize the file path to match stored paths
            let canonical_path = file_path.canonicalize().unwrap_or_else(|_| file_path.clone());
            let relative = canonical_path.strip_prefix(&self.root).unwrap_or(&canonical_path);

            match self.files.get(relative) {
                Some(file_state) => {
                    if file_state.has_changed(&self.root) {
                        changes.modified.push(file_path.clone());
                    } else {
                        changes.unchanged.push(file_path.clone());
                    }
                }
                None => {
                    changes.added.push(file_path.clone());
                }
            }
        }

        // Check for deleted files
        let current_set: std::collections::HashSet<_> = current_files
            .iter()
            .filter_map(|p| {
                let canonical = p.canonicalize().ok()?;
                Some(canonical.strip_prefix(&self.root).ok()?.to_path_buf())
            })
            .collect();

        for indexed_path in self.files.keys() {
            if !current_set.contains(indexed_path) {
                changes.deleted.push(self.root.join(indexed_path));
            }
        }

        changes
    }

    /// Update the last_updated timestamp
    pub fn touch(&mut self) {
        self.last_updated = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);
    }
}

/// Set of file changes detected
#[derive(Debug, Default, Clone)]
pub struct ChangeSet {
    /// Files that are new
    pub added: Vec<PathBuf>,
    /// Files that have been modified
    pub modified: Vec<PathBuf>,
    /// Files that have been deleted
    pub deleted: Vec<PathBuf>,
    /// Files that are unchanged
    pub unchanged: Vec<PathBuf>,
}

impl ChangeSet {
    /// Check if there are any changes
    pub fn has_changes(&self) -> bool {
        !self.added.is_empty() || !self.modified.is_empty() || !self.deleted.is_empty()
    }

    /// Get all files that need to be re-indexed
    pub fn files_to_index(&self) -> Vec<&PathBuf> {
        self.added.iter().chain(self.modified.iter()).collect()
    }

    /// Total number of changes
    pub fn total_changes(&self) -> usize {
        self.added.len() + self.modified.len() + self.deleted.len()
    }
}

impl Default for IndexState {
    fn default() -> Self {
        Self::new(PathBuf::new())
    }
}

