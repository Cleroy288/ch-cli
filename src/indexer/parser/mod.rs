//! Tree-sitter parser wrapper for Rust code.
//!
//! This module provides a high-level interface for parsing Rust source files
//! and extracting symbols and references using Tree-sitter queries.
//!
//! # Module Structure
//! - `types` - Core types (ExtractedReference, ParseError, Result)
//! - `helpers` - Utility functions (parse_visibility, is_rust_keyword)
//! - `doc_extraction` - Doc comment extraction functions
//! - `symbol_processing` - Symbol extraction from tree-sitter matches
//! - `reference_processing` - Reference extraction from tree-sitter matches
//! - `rust_parser` - Main RustParser struct
//! - `tests` - Unit tests

mod doc_extraction;
mod helpers;
mod parser_methods;
mod reference_processing;
mod rust_parser;
mod symbol_building;
mod symbol_processing;
mod types;

// Re-export public types for backward compatibility
pub use helpers::{is_rust_keyword, parse_visibility};
pub use rust_parser::RustParser;
pub use types::{ExtractedReference, ParseError, Result};
