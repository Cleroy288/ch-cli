//! Query rewriter submodule

mod decompose;
mod extract;
mod mapping;
mod rewriter;

pub use rewriter::{QueryRewriter, RewriteType, RewrittenQuery};

// Re-export submodule functions for testing
#[doc(hidden)]
pub use decompose::decompose_query;
#[doc(hidden)]
pub use extract::extract_symbols;
#[doc(hidden)]
pub use mapping::map_concepts;
