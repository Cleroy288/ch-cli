//! LLM Response Parsing
//!
//! Parses LLM JSON responses into SearchSpec and provides
//! a fallback parser for non-JSON responses.

mod core;
mod fallback;
mod json;

pub use core::{
	fallback_parse, parse_llm_response,
	QUERY_EXPANSION_PROMPT,
};

