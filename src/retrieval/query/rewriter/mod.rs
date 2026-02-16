//! Query rewriter submodule

mod decompose;
mod extract;
mod mapping;
mod core;
pub mod prf;

pub use core::{QueryRewriter, RewriteType, RewrittenQuery};

// Re-export submodule functions for testing
#[doc(hidden)]
pub use decompose::decompose_query;
#[doc(hidden)]
pub use extract::extract_symbols;
#[doc(hidden)]
pub use mapping::map_concepts;
#[doc(hidden)]
pub use prf::{
	expand_iterative, expand_with_feedback,
	extract_feedback_terms,
	split_to_words as prf_split_to_words,
	PrfExpansion, PrfFeedback, MAX_PRF_ITERATIONS,
};
