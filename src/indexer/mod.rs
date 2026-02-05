//! Semantic Code Indexer Module
//!
//! This module provides semantic code indexing capabilities using Tree-sitter
//! for parsing and symbol extraction, Tantivy for full-text search, and
//! semantic analysis for name resolution.
//!
//! # Architecture
//!
//! - `symbols`: Core data types (Symbol, SymbolKind, CodeLocation)
//! - `parser`: Tree-sitter parser wrapper and query execution
//! - `queries`: Tree-sitter query definitions for each language
//! - `crawler`: File system traversal with .gitignore support
//! - `manager`: High-level orchestration for indexing projects
//! - `search`: Tantivy-based full-text search engine
//! - `semantic`: Definition/reference tracking and name resolution
//!
//! # Example - Parse a single file
//!
//! ```ignore
//! use ch_cli::indexer::RustParser;
//!
//! let mut parser = RustParser::new()?;
//! let symbols = parser.parse_file("src/main.rs")?;
//!
//! for symbol in symbols {
//!     println!("{}: {} at line {}", symbol.kind, symbol.name, symbol.location.line);
//! }
//! ```
//!
//! # Example - Index and search a project
//!
//! ```ignore
//! use ch_cli::indexer::{IndexManager, SearchIndex};
//!
//! let manager = IndexManager::new();
//! let result = manager.index_project(".");
//!
//! // Create search index and add symbols
//! let search = SearchIndex::in_memory()?;
//! search.index_symbols(&result.symbols)?;
//!
//! // Search for symbols
//! let hits = search.search("handle", 10)?;
//! for hit in hits {
//!     println!("{}: {} (score: {})", hit.symbol.kind, hit.symbol.name, hit.score);
//! }
//! ```

pub mod analyzer;
pub mod crawler;
pub mod doc_parser;
pub mod manager;
pub mod parser;
pub mod queries;
pub mod search;
pub mod semantic;
pub mod state;
pub mod symbols;
pub mod trigram;
pub mod triple_search;
pub mod watcher;

// Re-export commonly used types
pub use analyzer::{CodebaseAnalysis, CodebaseAnalyzer, ProjectType};
pub use crawler::{CrawlStats, Crawler, CrawlerConfig, DetectedLanguage, FileResult, Language};
pub use doc_parser::DocParser;
pub use manager::{IndexError, IndexManager, IndexManagerResult, IndexResult, IndexStats};
pub use parser::{ExtractedReference, RustParser};
pub use search::{SearchError, SearchHit, SearchIndex};
pub use semantic::{
    AllUsages, Definition, ReferenceContext, ResolutionResult, SemanticGraph, SemanticStats,
    SymbolReference,
};
pub use state::{ChangeSet, FileState, IndexState, INDEX_DIR_NAME};
pub use symbols::{CodeLocation, ContentType, DocumentType, Symbol, SymbolKind, Visibility};
pub use trigram::{TrigramIndex, TrigramStats};
pub use triple_search::{TripleIndexStats, TripleSearchIndex, TripleSearchResults};
pub use watcher::{ChangeKind, FileChangeEvent, FileWatcher, WatcherError};

