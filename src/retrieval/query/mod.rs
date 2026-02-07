//! Query Expansion Module
//!
//! Uses a local LLM (Phi-3-mini) to interpret natural
//! language queries and convert them into structured
//! SearchSpec for code search.

mod caller;
mod fast_path;
pub mod fast_path_patterns;
pub mod interpreter;
pub mod llm;
mod llm_parsing;
pub mod parsing;
mod rewriter;
pub mod structure;
pub mod structure_search;
mod tiered;
mod tiered_strategies;

// Re-export internal functions for testing
#[doc(hidden)]
pub use tiered_strategies::{
	detect_intent as tiered_detect_intent,
	build_fast_path_spec_impl,
};
pub mod validator;
pub mod validator_scoring;

pub use caller::{
	detect_caller_query, CallerDirection, CallerQuery,
};
pub use fast_path::{
	FastPathIntent, FastPathParser, FastPathResult,
	SymbolCandidate,
};
pub use interpreter::QueryInterpreter;
pub use llm::Phi3Model;
pub use llm_parsing::{
	fallback_parse, parse_llm_response,
	QUERY_EXPANSION_PROMPT,
};
pub use parsing::{
	contains_word, extract_identifiers, filter_stop_words,
	parse_intent,
};
pub use rewriter::{
	QueryRewriter, RewriteType, RewrittenQuery,
};
// Re-export rewriter internals for testing
#[doc(hidden)]
pub use rewriter::{
	decompose_query, extract_symbols as rewriter_extract_symbols,
	map_concepts,
};
pub use structure::{
	detect_structure_query, StructureQuery,
};
pub use structure_search::{
	find_module_structure, list_source_directories,
	ModuleInfo, SubmoduleDecl,
};
pub use tiered::{
	TierUsed, TieredConfig, TieredQueryExpander,
	TieredResult,
};
pub use validator::{
	SymbolValidator, ValidatedSymbol, ValidationResult,
};
