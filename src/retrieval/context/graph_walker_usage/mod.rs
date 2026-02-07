//! Usage Types for Graph Walker
//!
//! Contains UsageInfo and UsageCollection types used
//! by GraphWalker to track symbol usages with deduplication.

mod collection_core;
mod collection_ops;

pub use collection_core::{UsageCollection, UsageInfo};

