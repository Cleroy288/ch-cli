mod conversion;
mod indexing;
mod persistence;
mod searching;
mod types;

// Re-export public types for backward compatibility
pub use conversion::{string_to_trigram, trigram_to_string};
pub use types::{Trigram, TrigramIndex, TrigramStats};
