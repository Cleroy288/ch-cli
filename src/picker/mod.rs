//! Picker module
//!
//! Provides file and folder picker functionality with filtering.
//!
//! # Modules
//! - `mode`: PickerMode enum representing picker states
//! - `state`: Picker struct managing picker state and operations
//! - `queries`: Query and selection management
//! - `selection`: Selection management operations
//! - `scanner`: Filesystem scanning operations

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

// Re-export commonly used types
pub use mode::PickerMode;
pub use queries::PickerQuery;
pub use scanner::PickerScanner;
pub use state::Picker;
