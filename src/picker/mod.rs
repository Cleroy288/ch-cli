//! Picker module
//!
//! Provides file/folder browsing, symbol drilling,
//! tools selection, and documentation browsing.
//!
//! # Modules
//! - `mode`: PickerMode enum representing picker states
//! - `state`: Picker struct managing picker state
//! - `queries`: Query and selection management
//! - `scanner`: Filesystem scanning operations
//! - `symbol_browser`: Symbol drilling into files
//! - `doc_browser`: Documentation entry browsing

pub mod doc_browser;
pub mod mode;
pub mod queries;
mod query_getters;
pub mod scanner;
mod selection;
pub mod state;
mod state_getters;
mod state_mode;
mod state_query;
mod state_results;
mod state_tools;
pub mod symbol_browser;

// Re-export commonly used types
pub use doc_browser::DocBrowser;
pub use mode::PickerMode;
pub use queries::PickerQuery;
pub use scanner::PickerScanner;
pub use state::Picker;
pub use symbol_browser::SymbolBrowser;
