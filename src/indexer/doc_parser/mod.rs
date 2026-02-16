//! Documentation Parser for Markdown Files
//!
//! Parses markdown files into semantic chunks for search indexing.
//! Each header section becomes a searchable DocumentChunk symbol.

mod chunk;
mod chunk_extract;
mod parser;

// Re-export public types for backward compatibility
pub use chunk::DocChunk;
pub use parser::DocParser;
