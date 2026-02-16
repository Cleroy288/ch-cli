//! Retrieval Pipeline
//!
//! Orchestrates the full code retrieval workflow from natural language
//! query to formatted context output.

mod config;
mod core;
mod formatting;
mod hybrid_init;
mod initialization;
mod initialization_helpers;
pub mod triple_init;
mod triple_init_cache;
mod persistence;
mod query_expansion;
mod search;
mod structured;
mod structured_builders;

// Re-export public types
pub use config::{
	PipelineConfig, PipelineFlags, PipelineResult,
};

// Re-export RetrievalPipeline from core module
pub use core::RetrievalPipeline;

