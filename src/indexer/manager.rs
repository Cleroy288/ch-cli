//! Index Manager - High-level orchestration for semantic indexing.
//!
//! This module provides the main entry point for indexing entire projects,
//! coordinating the crawler, parser, search index, and semantic analysis.
//!
//! Supports both full and incremental indexing with persistent storage.
//!
//! # Features
//!
//! - **Incremental parsing**: Only re-parse changed files using mtime tracking
//! - **Persistent storage**: Save index state and Tantivy index to disk
//! - **Reference extraction**: Extract symbol usages from AST (calls, types, imports)
//! - **File watching**: Auto-reindex on file changes
//!
//! # Example - Incremental indexing with persistence
//!
//! ```ignore
//! use ch_cli::indexer::IndexManager;
//!
//! let manager = IndexManager::new()
//!     .with_persistence()
//!     .with_semantic_analysis()
//!     .with_reference_extraction();
//!
//! // First run: full index
//! let result = manager.index_project(".")?;
//!
//! // Subsequent runs: incremental (only changed files)
//! let result = manager.index_project(".")?;
//! println!("Incremental: {} files changed", result.changes.map(|c| c.total_changes()).unwrap_or(0));
//! ```

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use rayon::prelude::*;

use crate::indexer::crawler::{CrawlStats, Crawler, CrawlerConfig, FileResult, Language};
use crate::indexer::doc_parser::DocParser;
use crate::indexer::parser::{ExtractedReference, RustParser};
use crate::indexer::search::SearchIndex;
use crate::indexer::semantic::{SemanticGraph, SymbolReference};
use crate::indexer::state::{ChangeSet, IndexState};
use crate::indexer::watcher::{FileWatcher, WatcherError};
use crate::indexer::Symbol;

/// Progress callback type for tracking indexing progress
pub type ProgressCallback = Box<dyn Fn(usize, usize, &Path) + Send + Sync>;

/// Callback for file change events during watch mode
pub type WatchCallback = Box<dyn Fn(&[PathBuf]) + Send + Sync>;

/// Error type for index manager operations
#[derive(Debug, thiserror::Error)]
pub enum IndexError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Search error: {0}")]
    Search(#[from] crate::indexer::search::SearchError),

    #[error("Watcher error: {0}")]
    Watcher(#[from] WatcherError),

    #[error("Parser error: {0}")]
    Parser(String),
}

pub type IndexManagerResult<T> = std::result::Result<T, IndexError>;

/// Result of indexing an entire project
pub struct IndexResult {
    /// Root directory that was indexed
    pub root: PathBuf,
    /// All symbols found across all files
    pub symbols: Vec<Symbol>,
    /// All references found across all files
    pub references: Vec<ExtractedReference>,
    /// Per-file results
    pub file_results: Vec<FileResult>,
    /// Statistics about the indexing operation
    pub stats: CrawlStats,
    /// Semantic graph for name resolution (if semantic analysis was enabled)
    pub semantic_graph: Option<SemanticGraph>,
    /// Whether this was an incremental index
    pub incremental: bool,
    /// Change set if incremental indexing was used
    pub changes: Option<ChangeSet>,
    /// Search index (if persistence was enabled)
    pub search_index: Option<SearchIndex>,
}

impl std::fmt::Debug for IndexResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("IndexResult")
            .field("root", &self.root)
            .field("symbols", &self.symbols.len())
            .field("references", &self.references.len())
            .field("file_results", &self.file_results.len())
            .field("stats", &self.stats)
            .field("semantic_graph", &self.semantic_graph.is_some())
            .field("incremental", &self.incremental)
            .field("changes", &self.changes.as_ref().map(|c| c.total_changes()))
            .field("search_index", &self.search_index.is_some())
            .finish()
    }
}

/// The main index manager for semantic code indexing
pub struct IndexManager {
    /// Crawler configuration
    crawler_config: CrawlerConfig,
    /// Optional progress callback
    progress_callback: Option<ProgressCallback>,
    /// Whether to build a semantic graph for name resolution
    enable_semantic_analysis: bool,
    /// Whether to extract references from AST
    enable_reference_extraction: bool,
    /// Whether to persist the index to disk (enables incremental indexing)
    enable_persistence: bool,
}

impl IndexManager {
    /// Create a new IndexManager with default configuration
    pub fn new() -> Self {
        Self {
            crawler_config: CrawlerConfig::default(),
            progress_callback: None,
            enable_semantic_analysis: false,
            enable_reference_extraction: false,
            enable_persistence: false,
        }
    }

    /// Create an IndexManager with custom crawler configuration
    pub fn with_config(config: CrawlerConfig) -> Self {
        Self {
            crawler_config: config,
            progress_callback: None,
            enable_semantic_analysis: false,
            enable_reference_extraction: false,
            enable_persistence: false,
        }
    }

    /// Enable semantic analysis for name resolution
    pub fn with_semantic_analysis(mut self) -> Self {
        self.enable_semantic_analysis = true;
        self
    }

    /// Enable reference extraction from AST
    pub fn with_reference_extraction(mut self) -> Self {
        self.enable_reference_extraction = true;
        self
    }

    /// Enable persistent storage (saves index to disk, enables incremental indexing)
    pub fn with_persistence(mut self) -> Self {
        self.enable_persistence = true;
        self
    }

    /// Set a progress callback
    pub fn on_progress<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize, usize, &Path) + Send + Sync + 'static,
    {
        self.progress_callback = Some(Box::new(callback));
        self
    }

    /// Index an entire project directory
    ///
    /// This method:
    /// 1. Discovers all indexable files using the crawler
    /// 2. Detects changes if persistence is enabled (incremental indexing)
    /// 3. Parses files in parallel using rayon (only changed files if incremental)
    /// 4. Extracts references if enabled
    /// 5. Builds semantic graph if enabled
    /// 6. Saves index state to disk if persistence is enabled
    ///
    /// Returns an IndexResult with all symbols, references, and optional search index.
    pub fn index_project<P: AsRef<Path>>(&self, root: P) -> IndexManagerResult<IndexResult> {
        let root = root.as_ref().canonicalize().unwrap_or_else(|_| root.as_ref().to_path_buf());
        let start_time = Instant::now();

        // Step 1: Discover all files
        let crawler = Crawler::with_config(self.crawler_config.clone());
        let all_files = crawler.discover_files(&root);
        let total_files = all_files.len();

        // Step 2: Check for existing index and detect changes (incremental)
        let (files_to_process, changes, mut index_state, is_incremental) = if self.enable_persistence {
            self.detect_changes(&root, &all_files)?
        } else {
            (all_files.clone(), None, IndexState::new(root.clone()), false)
        };

        let files_to_process_count = files_to_process.len();

        // Step 3: Parse files in parallel
        let processed = AtomicUsize::new(0);
        let symbols_count = AtomicUsize::new(0);
        let references_count = AtomicUsize::new(0);
        let failed_count = AtomicUsize::new(0);

        // Collect references in a thread-safe way if enabled
        let all_references: Arc<Mutex<Vec<ExtractedReference>>> = Arc::new(Mutex::new(Vec::new()));

        let file_results: Vec<FileResult> = files_to_process
            .par_iter()
            .map(|path| {
                // Parse symbols
                let result = self.parse_file(path);

                // Extract references if enabled
                if self.enable_reference_extraction {
                    if let Ok(refs) = self.parse_references(path) {
                        references_count.fetch_add(refs.len(), Ordering::SeqCst);
                        all_references.lock().unwrap().extend(refs);
                    }
                }

                // Update counters
                let current = processed.fetch_add(1, Ordering::SeqCst) + 1;
                if result.error.is_some() {
                    failed_count.fetch_add(1, Ordering::SeqCst);
                } else {
                    symbols_count.fetch_add(result.symbols.len(), Ordering::SeqCst);
                }

                // Call progress callback if set
                if let Some(ref callback) = self.progress_callback {
                    callback(current, files_to_process_count, path);
                }

                result
            })
            .collect();

        // Step 4: Collect all symbols from parsed files
        let new_symbols: Vec<Symbol> = file_results
            .iter()
            .flat_map(|r| r.symbols.clone())
            .collect();

        // Step 5: Handle incremental - merge with unchanged files' symbols if needed
        let (all_symbols, search_index) = if self.enable_persistence {
            self.merge_and_persist(&root, &new_symbols, &file_results, &changes, &mut index_state)?
        } else {
            (new_symbols, None)
        };

        // Step 6: Build semantic graph if enabled
        let references = match Arc::try_unwrap(all_references) {
            Ok(mutex) => mutex.into_inner().unwrap_or_default(),
            Err(arc) => arc.lock().unwrap().clone(),
        };

        let semantic_graph = if self.enable_semantic_analysis {
            let mut graph = SemanticGraph::new();
            graph.add_symbols(&all_symbols);

            // Add references to semantic graph
            for reference in &references {
                let sym_ref = SymbolReference {
                    name: reference.name.clone(),
                    location: reference.location.clone(),
                    context: reference.context.clone(),
                };
                graph.add_reference(sym_ref);
            }

            Some(graph)
        } else {
            None
        };

        let duration_ms = start_time.elapsed().as_millis() as u64;

        Ok(IndexResult {
            root: root.to_path_buf(),
            symbols: all_symbols,
            references,
            file_results,
            stats: CrawlStats {
                files_found: total_files,
                files_parsed: files_to_process_count - failed_count.load(Ordering::SeqCst),
                files_failed: failed_count.load(Ordering::SeqCst),
                symbols_found: symbols_count.load(Ordering::SeqCst),
                duration_ms,
            },
            semantic_graph,
            incremental: is_incremental,
            changes,
            search_index,
        })
    }

    /// Detect changes between current files and stored index state
    fn detect_changes(
        &self,
        root: &Path,
        current_files: &[PathBuf],
    ) -> IndexManagerResult<(Vec<PathBuf>, Option<ChangeSet>, IndexState, bool)> {
        if IndexState::exists(root) {
            // Load existing state
            let state = IndexState::load(root)?;
            let changes = state.detect_changes(current_files);

            if changes.has_changes() {
                // Incremental: only process changed files
                let files_to_process: Vec<PathBuf> = changes.files_to_index().into_iter().cloned().collect();
                Ok((files_to_process, Some(changes), state, true))
            } else {
                // No changes - still return the state but nothing to process
                Ok((Vec::new(), Some(changes), state, true))
            }
        } else {
            // No existing index - full reindex
            Ok((current_files.to_vec(), None, IndexState::new(root.to_path_buf()), false))
        }
    }

    /// Merge new symbols with existing index and persist to disk
    fn merge_and_persist(
        &self,
        root: &Path,
        new_symbols: &[Symbol],
        file_results: &[FileResult],
        changes: &Option<ChangeSet>,
        index_state: &mut IndexState,
    ) -> IndexManagerResult<(Vec<Symbol>, Option<SearchIndex>)> {
        let tantivy_path = IndexState::tantivy_dir(root);
        let search_index = SearchIndex::open_or_create(&tantivy_path)?;

        // Collect files to delete and file results for batch update
        let mut files_to_delete: Vec<PathBuf> = Vec::new();

        // If incremental, collect deleted files
        if let Some(ref change_set) = changes {
            // Add deleted files to the delete list
            files_to_delete.extend(change_set.deleted.iter().cloned());

            // Update index state for deleted files
            for deleted_path in &change_set.deleted {
                index_state.remove_file(deleted_path);
            }
        }

        // Collect successful file results for batch update
        let file_symbols: Vec<(PathBuf, Vec<Symbol>)> = file_results
            .iter()
            .filter(|r| r.error.is_none())
            .map(|r| (r.path.clone(), r.symbols.clone()))
            .collect();

        // Update index state for processed files
        for result in file_results {
            if result.error.is_none() {
                index_state.update_file(&result.path, result.symbols.len())?;
            }
        }

        // Batch update Tantivy index (single commit!)
        search_index.batch_update(&files_to_delete, &file_symbols)?;

        // Update timestamp and save state
        index_state.touch();
        index_state.save()?;

        Ok((new_symbols.to_vec(), Some(search_index)))
    }

    /// Parse references from a file
    fn parse_references(&self, path: &Path) -> Result<Vec<ExtractedReference>, String> {
        let language = Language::from_path(path);

        match language {
            Some(Language::Rust) => {
                let mut parser = RustParser::new().map_err(|e| e.to_string())?;
                parser.parse_file_references(path).map_err(|e| e.to_string())
            }
            Some(Language::Markdown) => Ok(Vec::new()), // no references in markdown
            None => Ok(Vec::new()),
        }
    }

    /// Parse a single file
    fn parse_file(&self, path: &Path) -> FileResult {
        // Determine language and create appropriate parser
        let language = Language::from_path(path);

        match language {
            Some(Language::Rust) => {
                self.parse_rust_file(path)
            }
            Some(Language::Markdown) => {
                // Parse markdown into document chunks
                match self.parse_markdown(path) {
                    Ok(symbols) => FileResult {
                        path: path.to_path_buf(),
                        symbols,
                        error: None,
                    },
                    Err(_) => FileResult {
                        path: path.to_path_buf(),
                        symbols: Vec::new(),
                        error: None,
                    },
                }
            }
            None => FileResult {
                path: path.to_path_buf(),
                symbols: Vec::new(),
                error: None,
            },
        }
    }

    /// Parse markdown file into document chunks by headers
    /// Uses DocParser for hierarchical section extraction
    fn parse_markdown(&self, path: &Path) -> std::io::Result<Vec<Symbol>> {
        let content = std::fs::read_to_string(path)?;
        Ok(DocParser::parse(path, &content))
    }

    /// Parse Rust file with doc comment extraction
    /// Enriches symbols with their associated doc comments
    fn parse_rust_file(&self, path: &Path) -> FileResult {
        // create parser
        let mut parser = match RustParser::new() {
            Ok(p) => p,
            Err(e) => {
                return FileResult {
                    path: path.to_path_buf(),
                    symbols: Vec::new(),
                    error: Some(e),
                };
            }
        };

        // read source for doc extraction
        let source = match std::fs::read_to_string(path) {
            Ok(s) => s,
            Err(e) => {
                return FileResult {
                    path: path.to_path_buf(),
                    symbols: Vec::new(),
                    error: Some(crate::indexer::parser::ParseError::IoError(e)),
                };
            }
        };

        // parse symbols
        let mut symbols = match parser.parse_source(&source, path) {
            Ok(s) => s,
            Err(e) => {
                return FileResult {
                    path: path.to_path_buf(),
                    symbols: Vec::new(),
                    error: Some(e),
                };
            }
        };

        // extract doc comments for each symbol
        for symbol in &mut symbols {
            if let Some(doc) = RustParser::extract_item_doc(&source, symbol.location.line) {
                symbol.doc_comment = Some(doc);
            }
        }

        // extract module docs and create module symbol
        if let Some(module_doc) = RustParser::extract_module_docs(&source) {
            let module_name = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("module");

            let module_symbol = Symbol::new(
                format!("{}_docs", module_name),
                crate::indexer::SymbolKind::DocumentChunk,
                crate::indexer::CodeLocation::new(path.to_path_buf(), 1, 1, 0, 0),
            )
            .with_content(module_doc.clone())
            .with_doc_comment(module_doc);

            symbols.push(module_symbol);
        }

        FileResult {
            path: path.to_path_buf(),
            symbols,
            error: None,
        }
    }

    /// Watch a project directory for changes and auto-reindex
    ///
    /// This method:
    /// 1. Performs an initial full/incremental index
    /// 2. Starts watching for file changes
    /// 3. Re-indexes changed files automatically
    /// 4. Calls the optional callback when changes are detected
    ///
    /// The watch loop runs until `stop_flag` returns true.
    ///
    /// # Example
    ///
    /// ```ignore
    /// use std::sync::atomic::{AtomicBool, Ordering};
    /// use std::sync::Arc;
    ///
    /// let manager = IndexManager::new()
    ///     .with_persistence()
    ///     .with_semantic_analysis();
    ///
    /// let stop_flag = Arc::new(AtomicBool::new(false));
    /// let stop_flag_clone = stop_flag.clone();
    ///
    /// // In another thread: stop_flag_clone.store(true, Ordering::SeqCst);
    ///
    /// manager.watch_project(".", || stop_flag.load(Ordering::SeqCst), |paths| {
    ///     println!("Reindexed {} files", paths.len());
    /// })?;
    /// ```
    pub fn watch_project<P, F, C>(
        &self,
        root: P,
        stop_flag: F,
        on_change: C,
    ) -> IndexManagerResult<IndexResult>
    where
        P: AsRef<Path>,
        F: Fn() -> bool,
        C: Fn(&[PathBuf]),
    {
        let root = root.as_ref().canonicalize().unwrap_or_else(|_| root.as_ref().to_path_buf());

        // Enable persistence for watch mode
        let manager_with_persistence = IndexManager {
            crawler_config: self.crawler_config.clone(),
            progress_callback: None, // Don't use progress in watch mode
            enable_semantic_analysis: self.enable_semantic_analysis,
            enable_reference_extraction: self.enable_reference_extraction,
            enable_persistence: true, // Force persistence for watch mode
        };

        // Initial index
        let mut result = manager_with_persistence.index_project(&root)?;

        // Start file watcher
        let mut watcher = FileWatcher::new(&root)?;
        watcher.start()?;

        // Watch loop with debouncing
        let debounce_duration = Duration::from_millis(500);
        let mut pending_changes: Vec<PathBuf> = Vec::new();
        let mut last_change_time: Option<Instant> = None;

        while !stop_flag() {
            // Poll for changes with timeout
            match watcher.wait(Duration::from_millis(100)) {
                Ok(Some(event)) => {
                    // Accumulate changes
                    for path in event.paths {
                        if !pending_changes.contains(&path) {
                            pending_changes.push(path);
                        }
                    }
                    last_change_time = Some(Instant::now());
                }
                Ok(None) => {
                    // Check if we should process pending changes (debounce)
                    if let Some(last_time) = last_change_time {
                        if last_time.elapsed() >= debounce_duration && !pending_changes.is_empty() {
                            // Process accumulated changes
                            let changed_paths = std::mem::take(&mut pending_changes);
                            last_change_time = None;

                            // Re-index the project (incremental)
                            result = manager_with_persistence.index_project(&root)?;

                            // Notify callback
                            on_change(&changed_paths);
                        }
                    }
                }
                Err(WatcherError::Timeout) => continue,
                Err(e) => return Err(e.into()),
            }
        }

        // Stop watcher
        let _ = watcher.stop();

        Ok(result)
    }

    /// Index specific files only (for targeted re-indexing)
    ///
    /// Useful for re-indexing a subset of files without crawling the entire project.
    pub fn index_files<P: AsRef<Path>>(&self, root: P, files: &[PathBuf]) -> IndexManagerResult<IndexResult> {
        let root = root.as_ref().canonicalize().unwrap_or_else(|_| root.as_ref().to_path_buf());
        let start_time = Instant::now();

        let processed = AtomicUsize::new(0);
        let symbols_count = AtomicUsize::new(0);
        let references_count = AtomicUsize::new(0);
        let failed_count = AtomicUsize::new(0);

        let all_references: Arc<Mutex<Vec<ExtractedReference>>> = Arc::new(Mutex::new(Vec::new()));

        let file_results: Vec<FileResult> = files
            .par_iter()
            .map(|path| {
                let result = self.parse_file(path);

                if self.enable_reference_extraction {
                    if let Ok(refs) = self.parse_references(path) {
                        references_count.fetch_add(refs.len(), Ordering::SeqCst);
                        all_references.lock().unwrap().extend(refs);
                    }
                }

                let current = processed.fetch_add(1, Ordering::SeqCst) + 1;
                if result.error.is_some() {
                    failed_count.fetch_add(1, Ordering::SeqCst);
                } else {
                    symbols_count.fetch_add(result.symbols.len(), Ordering::SeqCst);
                }

                if let Some(ref callback) = self.progress_callback {
                    callback(current, files.len(), path);
                }

                result
            })
            .collect();

        let all_symbols: Vec<Symbol> = file_results
            .iter()
            .flat_map(|r| r.symbols.clone())
            .collect();

        let references = match Arc::try_unwrap(all_references) {
            Ok(mutex) => mutex.into_inner().unwrap_or_default(),
            Err(arc) => arc.lock().unwrap().clone(),
        };

        let semantic_graph = if self.enable_semantic_analysis {
            let mut graph = SemanticGraph::new();
            graph.add_symbols(&all_symbols);

            for reference in &references {
                let sym_ref = SymbolReference {
                    name: reference.name.clone(),
                    location: reference.location.clone(),
                    context: reference.context.clone(),
                };
                graph.add_reference(sym_ref);
            }

            Some(graph)
        } else {
            None
        };

        let duration_ms = start_time.elapsed().as_millis() as u64;

        Ok(IndexResult {
            root: root.to_path_buf(),
            symbols: all_symbols,
            references,
            file_results,
            stats: CrawlStats {
                files_found: files.len(),
                files_parsed: files.len() - failed_count.load(Ordering::SeqCst),
                files_failed: failed_count.load(Ordering::SeqCst),
                symbols_found: symbols_count.load(Ordering::SeqCst),
                duration_ms,
            },
            semantic_graph,
            incremental: false,
            changes: None,
            search_index: None,
        })
    }

    /// Clear the index for a project (delete all persisted data)
    pub fn clear_index<P: AsRef<Path>>(root: P) -> IndexManagerResult<()> {
        let root = root.as_ref();
        let index_dir = IndexState::index_dir(root);

        if index_dir.exists() {
            std::fs::remove_dir_all(&index_dir)?;
        }

        Ok(())
    }

    /// Check if an index exists for the given project
    pub fn has_index<P: AsRef<Path>>(root: P) -> bool {
        IndexState::exists(root.as_ref())
    }

    /// Get index statistics without performing a full reindex
    pub fn get_index_stats<P: AsRef<Path>>(root: P) -> IndexManagerResult<Option<IndexStats>> {
        let root = root.as_ref();

        if !IndexState::exists(root) {
            return Ok(None);
        }

        let state = IndexState::load(root)?;
        let tantivy_path = IndexState::tantivy_dir(root);

        let symbol_count = if tantivy_path.exists() {
            SearchIndex::open_or_create(&tantivy_path)?
                .num_docs()
                .unwrap_or(0)
        } else {
            0
        };

        Ok(Some(IndexStats {
            root: root.to_path_buf(),
            file_count: state.files.len(),
            symbol_count: symbol_count as usize,
            last_updated: state.last_updated,
            version: state.version,
        }))
    }
}

/// Statistics about a persisted index
#[derive(Debug, Clone)]
pub struct IndexStats {
    /// Project root
    pub root: PathBuf,
    /// Number of indexed files
    pub file_count: usize,
    /// Number of indexed symbols
    pub symbol_count: usize,
    /// Last update timestamp (seconds since UNIX epoch)
    pub last_updated: u64,
    /// Index format version
    pub version: u32,
}

impl Default for IndexManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::indexer::SymbolKind;
    use std::fs;
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_project() -> TempDir {
        let temp_dir = TempDir::new().expect("Failed to create temp dir");

        // Create a simple Rust file
        let src_dir = temp_dir.path().join("src");
        fs::create_dir_all(&src_dir).expect("Failed to create src dir");

        let lib_rs = src_dir.join("lib.rs");
        let mut file = fs::File::create(&lib_rs).expect("Failed to create lib.rs");
        writeln!(
            file,
            r#"
pub struct MyStruct {{
    pub field: i32,
}}

impl MyStruct {{
    pub fn new() -> Self {{
        Self {{ field: 0 }}
    }}

    pub fn get_field(&self) -> i32 {{
        self.field
    }}
}}

pub fn helper_function() -> i32 {{
    42
}}

pub enum Status {{
    Active,
    Inactive,
}}
"#
        )
        .expect("Failed to write lib.rs");

        // Create another file
        let main_rs = src_dir.join("main.rs");
        let mut file = fs::File::create(&main_rs).expect("Failed to create main.rs");
        writeln!(
            file,
            r#"
mod lib;

fn main() {{
    println!("Hello");
}}
"#
        )
        .expect("Failed to write main.rs");

        temp_dir
    }

    #[test]
    fn test_index_project_basic() {
        let temp_dir = create_test_project();

        let manager = IndexManager::new();
        let result = manager.index_project(temp_dir.path()).unwrap();

        assert_eq!(result.stats.files_found, 2);
        assert_eq!(result.stats.files_parsed, 2);
        assert_eq!(result.stats.files_failed, 0);
        assert!(result.stats.symbols_found > 0);
        assert!(result.semantic_graph.is_none()); // Not enabled
    }

    #[test]
    fn test_index_project_with_semantic_analysis() {
        let temp_dir = create_test_project();

        let manager = IndexManager::new().with_semantic_analysis();
        let result = manager.index_project(temp_dir.path()).unwrap();

        assert!(result.semantic_graph.is_some());

        let graph = result.semantic_graph.as_ref().unwrap();
        let stats = graph.stats();

        assert!(stats.total_definitions > 0);
        assert!(stats.unique_symbols > 0);
        assert_eq!(stats.files_analyzed, 2);
    }

    #[test]
    fn test_semantic_graph_finds_struct() {
        let temp_dir = create_test_project();

        let manager = IndexManager::new().with_semantic_analysis();
        let result = manager.index_project(temp_dir.path()).unwrap();

        let graph = result.semantic_graph.as_ref().unwrap();

        // Find the struct we defined (may also find impl block reference)
        let defs = graph.find_definitions("MyStruct");
        assert!(!defs.is_empty());
        // At least one should be a Struct
        assert!(defs.iter().any(|d| d.symbol.kind == SymbolKind::Struct));
    }

    #[test]
    fn test_semantic_graph_finds_methods() {
        let temp_dir = create_test_project();

        let manager = IndexManager::new().with_semantic_analysis();
        let result = manager.index_project(temp_dir.path()).unwrap();

        let graph = result.semantic_graph.as_ref().unwrap();

        // Find the "new" method
        let defs = graph.find_definitions("new");
        assert!(!defs.is_empty());

        // Find the "get_field" method
        let defs = graph.find_definitions("get_field");
        assert!(!defs.is_empty());
    }

    #[test]
    fn test_semantic_graph_finds_enum() {
        let temp_dir = create_test_project();

        let manager = IndexManager::new().with_semantic_analysis();
        let result = manager.index_project(temp_dir.path()).unwrap();

        let graph = result.semantic_graph.as_ref().unwrap();

        // Find the enum
        let defs = graph.find_definitions("Status");
        assert_eq!(defs.len(), 1);
        assert_eq!(defs[0].symbol.kind, SymbolKind::Enum);

        // Find enum variants
        let active = graph.find_definitions("Active");
        assert!(!active.is_empty());

        let inactive = graph.find_definitions("Inactive");
        assert!(!inactive.is_empty());
    }

    #[test]
    fn test_semantic_graph_find_by_kind() {
        let temp_dir = create_test_project();

        let manager = IndexManager::new().with_semantic_analysis();
        let result = manager.index_project(temp_dir.path()).unwrap();

        let graph = result.semantic_graph.as_ref().unwrap();

        // Find all structs
        let structs = graph.find_by_kind(SymbolKind::Struct);
        assert!(structs.iter().any(|d| d.symbol.name == "MyStruct"));

        // Find all enums
        let enums = graph.find_by_kind(SymbolKind::Enum);
        assert!(enums.iter().any(|d| d.symbol.name == "Status"));

        // Find all functions
        let functions = graph.find_by_kind(SymbolKind::Function);
        assert!(functions.iter().any(|d| d.symbol.name == "helper_function"));
    }

    #[test]
    fn test_index_result_debug() {
        let temp_dir = create_test_project();

        let manager = IndexManager::new().with_semantic_analysis();
        let result = manager.index_project(temp_dir.path()).unwrap();

        // Test that Debug trait works
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("IndexResult"));
        assert!(debug_str.contains("semantic_graph"));
    }

    #[test]
    fn test_incremental_indexing() {
        let temp_dir = create_test_project();

        // First index with persistence
        let manager = IndexManager::new().with_persistence();
        let result1 = manager.index_project(temp_dir.path()).unwrap();
        assert!(!result1.incremental); // First run is not incremental

        // Second index should be incremental with no changes
        let result2 = manager.index_project(temp_dir.path()).unwrap();
        assert!(result2.incremental);
        assert!(result2.changes.is_some());
        let changes = result2.changes.as_ref().unwrap();
        assert!(!changes.has_changes()); // No changes

        // Clean up
        IndexManager::clear_index(temp_dir.path()).unwrap();
    }

    #[test]
    fn test_reference_extraction() {
        let temp_dir = create_test_project();

        let manager = IndexManager::new()
            .with_semantic_analysis()
            .with_reference_extraction();
        let result = manager.index_project(temp_dir.path()).unwrap();

        // Should have extracted some references
        assert!(!result.references.is_empty());

        // Semantic graph should have references
        let graph = result.semantic_graph.as_ref().unwrap();
        let stats = graph.stats();
        assert!(stats.total_references > 0);
    }

    #[test]
    fn test_index_stats() {
        let temp_dir = create_test_project();

        // Initially no index
        assert!(!IndexManager::has_index(temp_dir.path()));

        // Create index with persistence
        let manager = IndexManager::new().with_persistence();
        let _ = manager.index_project(temp_dir.path()).unwrap();

        // Now should have index
        assert!(IndexManager::has_index(temp_dir.path()));

        // Get stats
        let stats = IndexManager::get_index_stats(temp_dir.path()).unwrap();
        assert!(stats.is_some());
        let stats = stats.unwrap();
        assert_eq!(stats.file_count, 2);
        assert!(stats.symbol_count > 0);

        // Clean up
        IndexManager::clear_index(temp_dir.path()).unwrap();
        assert!(!IndexManager::has_index(temp_dir.path()));
    }
}
