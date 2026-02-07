//! Type definitions for the parser module.
//!
//! Contains the core types used across the parser:
//! ExtractedReference, ParseError, and Result.

use crate::indexer::semantic::ReferenceContext;
use crate::indexer::symbols::CodeLocation;

// Re-export errors from domain for backward compat
pub use crate::domain::errors::parse::ParseError;

/// Result type alias for parser operations.
pub type Result<T> =
	std::result::Result<T, ParseError>;

/// A reference to a symbol extracted from the AST.
/// Represents a usage of a symbol
/// (function call, type reference, etc.)
#[derive(Debug, Clone)]
pub struct ExtractedReference {
	/// The name being referenced
	pub name: String,
	/// Location of the reference in the source file
	pub location: CodeLocation,
	/// The kind of reference (call, type, import, etc.)
	pub context: ReferenceContext,
}
