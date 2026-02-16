//! Memory system indexer — persistence and search.
//!
//! - `paths`: Directory and file path helpers
//! - `store`: JSONL append/load operations
//! - `store_helpers`: Low-level I/O utilities
//! - `store_filters`: File system filter helpers
//! - `search_schema`: Tantivy schema definition
//! - `search_index`: Tantivy index (struct + init)
//! - `search_index_ops`: Index/search/count methods
//! - `search_index_query`: Query execution helpers
//! - `search_convert`: Interaction → Document
//! - `search_extract`: Document → Interaction
//! - `types`: Result types (MemoryHit, MemoryStats)
//! - `stats`: Stats computation and caching

pub mod paths;
pub mod search_convert;
pub mod search_extract;
pub mod search_index;
mod search_index_ops;
mod search_index_query;
pub mod search_schema;
pub mod stats;
pub mod store;
mod store_filters;
pub mod store_helpers;
pub mod types;

pub use search_index::MemorySearchIndex;
pub use search_schema::{
	MemoryFields, build_memory_schema,
};
pub use stats::compute_stats;
pub use store::{append, load_recent, load_session};
pub use types::{MemoryHit, MemoryStats};
