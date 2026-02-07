//! Trigram Index for Fast Text Search
//!
//! Implements a trigram-based index for fast pre-filtering of files
//! before semantic search. Based on the approach used by:
//! - rust-analyzer (find usages)
//! - Zoekt (Sourcegraph code search)
//!
//! A trigram is a sequence of 3 consecutive characters.
//! The index maps each trigram to the set of files containing it.

mod conversion;
mod indexing;
mod persistence;
mod searching;
mod types;

// Re-export public types for backward compatibility
pub use conversion::{string_to_trigram, trigram_to_string};
pub use types::{Trigram, TrigramIndex, TrigramStats};
