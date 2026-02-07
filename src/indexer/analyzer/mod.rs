//! Codebase language analyzer.
//!
//! Detects the primary programming language(s) used in a codebase
//! by analyzing file extensions and project configuration files.

mod analysis;
mod analysis_core;
mod codebase_analyzer;
mod detection;
mod project_type;
mod project_type_core;
// Re-export all public types for backward compatibility
pub use analysis::CodebaseAnalysis;
pub use codebase_analyzer::CodebaseAnalyzer;
pub use project_type::ProjectType;
