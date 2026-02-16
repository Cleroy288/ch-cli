//! Symbol types for the semantic indexer.
//!
//! This module defines the core data structures used to represent
//! code symbols extracted from source files.

mod content_type;
mod document_type;
mod kind;
mod kind_boost;
mod location;
mod symbol;
mod symbol_builder;
mod visibility;

// Re-export all public types for backward compatibility
pub use content_type::ContentType;
pub use document_type::{is_test_file, DocumentType};
pub use kind::SymbolKind;
pub use location::{ByteSpan, CodeLocation};
pub use symbol::Symbol;
pub use visibility::Visibility;
