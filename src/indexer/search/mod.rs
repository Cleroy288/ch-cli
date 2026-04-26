mod conversion;
mod conversion_parsing;
mod conversion_read;
mod error;
mod index_core;
mod indexing;
mod loading;
mod querying;
mod querying_helpers;
mod schema;
mod schema_builder;
mod types;

// Re-export public types for backward compatibility
pub use conversion::symbol_to_doc;
pub use conversion_read::doc_to_symbol;
pub use error::{SearchError, SearchResult};
pub use index_core::SearchIndex;
pub use schema::{SchemaFields, build_schema};
pub use types::SearchHit;
