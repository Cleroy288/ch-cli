//! Documentation Generator Module
//!
//! This module provides automatic documentation generation
//! for code symbols using a local LLM. It stores both
//! user-written comments and LLM-generated descriptions,
//! along with comprehensive cross-references.
//!
//! ## Features
//!
//! - **Doc Generation**: Uses Qwen2.5-0.5B to generate descriptions
//! - **User Comments**: Preserves existing /// and //! comments
//! - **Cross-References**: Tracks where each symbol is used
//! - **Symbol Links**: Maps dependencies between symbols
//! - **Incremental Updates**: Only regenerates on file changes
//!
//! ## Architecture
//!
//! ```text
//! DocGenerator (LLM) --> DocEntry --> DocStore --> .rustean-index/docs.json
//!                           |
//!                           +-- user_comment
//!                           +-- llm_doc
//!                           +-- references[]
//!                           +-- links (depends_on, depended_by)
//! ```

pub mod doc_llm;
pub mod entry;
pub mod entry_builders;
pub mod entry_construct;
pub mod entry_status;
pub mod entry_text;
pub mod entry_types;
pub mod generator;
pub mod generator_batch;
pub mod generator_single;
pub mod generator_utils;
pub mod linker;
pub mod linker_analysis;
pub mod linker_paths;
pub mod linker_symbols;
pub mod prompts;
pub mod reference_location;
pub mod store_core;
pub mod store_entries;
pub mod store_persistence;
pub mod store_query;
pub mod store_sync;
pub mod symbol_links;

// Re-export entry types (preserves original public API)
pub use entry::DocEntry;
pub use entry_types::{DocStatus, ReferenceKind};
pub use reference_location::ReferenceLocation;
pub use symbol_links::SymbolLinks;

// Re-export generator
pub use generator::DocGenerator;

// Re-export linker
pub use linker::DocLinker;

// Re-export store (was `store::DocStore`)
pub use store_core::{DocStore, DocStoreStats};

// Re-export prompts
pub use prompts::{
	DOC_PROMPT_FUNCTION, DOC_PROMPT_STRUCT,
	DOC_PROMPT_TRAIT,
};
