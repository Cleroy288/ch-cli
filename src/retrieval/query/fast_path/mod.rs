//! Fast Path Parser submodule

mod extraction;
mod intent;
mod parser;
mod result;

// Re-export from fast_path_patterns
pub use super::fast_path_patterns::{
	FastPathIntent, FastPathResult, SymbolCandidate,
};
pub use parser::FastPathParser;

