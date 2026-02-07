//! Structure Query Detection
//!
//! Detects queries about module/directory structure and extracts
//! the target directory or module name.

mod core;
mod patterns;
mod utils;

pub use core::{detect_structure_query, StructureQuery};

