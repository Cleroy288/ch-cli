//! Documentation Generator Module
//!
//! This module provides automatic documentation generation for code symbols
//! using a local LLM. It stores both user-written comments and LLM-generated
//! descriptions, along with comprehensive cross-references.
//!
//! ## Features
//!
//! - **Doc Generation**: Uses Phi-3 LLM to generate descriptions
//! - **User Comments**: Preserves existing /// and //! comments
//! - **Cross-References**: Tracks where each symbol is used
//! - **Symbol Links**: Maps dependencies between symbols
//! - **Incremental Updates**: Only regenerates on file changes
//!
//! ## Architecture
//!
//! ```text
//! DocGenerator (LLM) ──► DocEntry ──► DocStore ──► .ch-index/docs.json
//!                           │
//!                           ├── user_comment
//!                           ├── llm_doc
//!                           ├── references[]
//!                           └── links (depends_on, depended_by)
//! ```

pub mod entry;
pub mod generator;
pub mod linker;
pub mod prompts;
pub mod store;

pub use entry::{DocEntry, DocStatus, ReferenceKind, ReferenceLocation, SymbolLinks};
pub use generator::DocGenerator;
pub use linker::DocLinker;
pub use prompts::{DOC_PROMPT_FUNCTION, DOC_PROMPT_STRUCT, DOC_PROMPT_TRAIT};
pub use store::DocStore;
