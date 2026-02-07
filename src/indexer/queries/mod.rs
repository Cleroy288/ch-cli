//! Tree-sitter query definitions for different languages.
//!
//! This module contains the S-expression queries used to extract
//! symbols from parsed ASTs.

mod doc_comments;
mod references;
mod symbols;

// Re-export all public constants for backward compatibility
pub use doc_comments::RUST_DOC_COMMENTS_QUERY;
pub use references::RUST_REFERENCES_QUERY;
pub use symbols::RUST_SYMBOLS_QUERY;
