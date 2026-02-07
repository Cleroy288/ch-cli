//! Agentic Code Retrieval Module
//!
//! This module provides an intelligent code retrieval system with:
//! - Query expansion via local LLM
//! - Hybrid search (keyword + semantic)
//! - Context expansion using SemanticGraph
//! - Semantic reranking with cross-encoder
//!
//! The AI models run as a background daemon to avoid reload latency.

pub mod agent;
pub mod context;
pub mod daemon;
pub mod docgen;
pub mod hybrid;
pub mod models;
pub mod query;
pub mod rerank;

pub use agent::{
	CodeResult, DocResult, FeedbackAction, FeedbackLoop, NotesResult, PipelineConfig,
	RetrievalOutput, RetrievalPipeline, StructuredOutput,
};
pub use context::{
	BlockBuilder, CalleeInfo, CallerInfo, ContextConfig, ContextExpander, ContextualBlock,
	GraphWalker, ParentContext, RelatedType, TypeRelationship,
};
pub use docgen::{DocEntry, DocGenerator, DocLinker, DocStatus, DocStore};
pub use query::{Phi3Model, QueryInterpreter};
pub use rerank::{BgeReranker, RerankedItem};

use std::path::PathBuf;

// Re-export errors from domain for backward compat
pub use crate::domain::errors::retrieval::{
	RetrievalError, RetrievalResult,
};

/// Configuration for the retrieval system
#[derive(Debug, Clone)]
pub struct RetrievalConfig {
	/// path to the daemon socket
	pub socket_path: PathBuf,
	/// path to the PID file
	pub pid_file: PathBuf,
	/// path to model cache directory
	pub model_cache: PathBuf,
	/// embedding model identifier
	pub embedding_model: String,
	/// reranker model identifier
	pub reranker_model: String,
	/// query expansion model identifier
	pub expansion_model: String,
}

impl Default for RetrievalConfig {
	/// Create default configuration using platform-specific directories
	fn default() -> Self {
		let base_dir = directories::ProjectDirs::from(
			"com", "ch-cli", "ch-cli"
		)
		.map(|d| d.data_dir().to_path_buf())
		.unwrap_or_else(|| PathBuf::from(".ch-cli"));

		Self {
			socket_path: base_dir.join("ml.sock"),
			pid_file: base_dir.join("daemon.pid"),
			model_cache: base_dir.join("models"),
			embedding_model: "BAAI/bge-small-en-v1.5".to_string(),
			reranker_model: "BAAI/bge-reranker-base".to_string(),
			expansion_model:
				"microsoft/phi-3-mini-4k-instruct".to_string(),
		}
	}
}
