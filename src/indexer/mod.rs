pub mod analyzer;
pub mod crawler;
pub mod doc_parser;
pub mod manager;
pub mod memory;
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
pub use crawler::{
	CrawlStats, Crawler, CrawlerConfig,
	DetectedLanguage, FileResult, Language,
};
pub use doc_parser::DocParser;
pub use manager::{
	IndexError, IndexManager, IndexManagerResult,
	IndexResult, IndexStats,
};
pub use parser::{ExtractedReference, RustParser};
pub use search::{SearchError, SearchHit, SearchIndex};
pub use semantic::{
	AllUsages, Definition, ReferenceContext,
	ResolutionResult, SemanticGraph,
	SemanticStats, SymbolReference,
};
pub use state::{ChangeSet, FileState, IndexState};
pub use symbols::{
	ByteSpan, CodeLocation, ContentType,
	DocumentType, Symbol, SymbolKind, Visibility,
};
pub use trigram::{TrigramIndex, TrigramStats};
pub use triple_search::{
	TripleIndexStats, TripleLimits,
	TripleSearchIndex, TripleSearchResults,
};
pub use watcher::{
	ChangeKind, FileChangeEvent, FileWatcher,
	WatcherError, WatcherResult,
};
pub use memory::{
	MemoryFields, MemoryHit, MemorySearchIndex,
	MemoryStats,
};

