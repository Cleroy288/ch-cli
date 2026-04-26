mod doc_comments;
mod references;
mod symbols;
mod ts_symbols;

// Re-export query constants
pub use doc_comments::RUST_DOC_COMMENTS_QUERY;
pub use references::RUST_REFERENCES_QUERY;
pub use symbols::RUST_SYMBOLS_QUERY;
pub use ts_symbols::TS_SYMBOLS_QUERY;
