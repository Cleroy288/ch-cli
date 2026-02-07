//! Retrieval Pipeline
//!
//! Orchestrates the full code retrieval workflow from natural language
//! query to formatted context output.

mod config;
mod core;
mod formatting;
mod hybrid_init;
mod initialization;
mod persistence;
mod search;
mod structured;
mod structured_builders;

// Re-export public types
pub use config::{PipelineConfig, PipelineResult};

// Re-export RetrievalPipeline from core module
pub use core::RetrievalPipeline;

