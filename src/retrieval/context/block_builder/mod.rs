//! Block Builder for Code Extraction
//!
//! Reads source files and extracts code snippets for symbols,
//! assembling them into ContextualBlocks.

mod core;
mod extractor;
mod expander;

pub use core::BlockBuilder;
pub use expander::ContextExpander;

