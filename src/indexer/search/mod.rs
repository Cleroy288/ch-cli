//! Tantivy-based search engine for the semantic indexer.
//!
//! This module provides full-text search capabilities for indexed symbols,
//! including fuzzy search, filtering by kind, and ranked results.
//!
//! # Structure
//! - `error.rs`: Error types and Result alias
//! - `types.rs`: SearchHit result type
//! - `schema.rs`: Tantivy schema definition and field handles
//! - `index_core.rs`: SearchIndex struct with creation/opening methods
//! - `indexing.rs`: Methods for adding/updating/removing symbols
//! - `querying.rs`: Search methods (search, fuzzy_search, search_by_kind)
//! - `conversion.rs`: Symbol <-> TantivyDocument conversion
//! - `tests.rs`: Unit tests

mod conversion;
mod conversion_parsing;
mod error;
mod index_core;
mod indexing;
mod loading;
mod querying;
mod schema;
mod schema_builder;
mod types;

// Re-export public types for backward compatibility
pub use conversion::{doc_to_symbol, symbol_to_doc};
pub use error::{SearchError, SearchResult};
pub use index_core::SearchIndex;
pub use schema::{SchemaFields, build_schema};
pub use types::SearchHit;
