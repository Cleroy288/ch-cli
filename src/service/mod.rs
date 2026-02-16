//! Service layer — use case orchestration.
//!
//! Each service defines a trait (for DI/mocking) and
//! a default implementation backed by infrastructure.
//!
//! Services import domain types only. Never import
//! handlers, never import database/framework code
//! directly.

pub mod claude;
pub mod daemon;
pub mod docgen;
pub mod index;
pub mod memory;
pub mod retrieval;
pub mod search;

pub use daemon::{DaemonService, DefaultDaemonService};
pub use docgen::{DefaultDocGenService, DocGenService};
pub use index::{DefaultIndexService, IndexService};
pub use memory::{
	DefaultMemoryService, MemoryService,
};
pub use retrieval::{
	DefaultRetrievalService, RetrievalService,
};
pub use search::{DefaultSearchService, SearchService};
