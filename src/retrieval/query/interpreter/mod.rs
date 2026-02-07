//! Query Interpreter
//!
//! Uses the Phi-3 LLM to interpret natural language queries
//! and convert them to structured SearchSpec.

mod core;
mod interpret;

pub use core::QueryInterpreter;

