//! Domain error types for all application layers.
//!
//! Each subdomain has its own error enum with `thiserror` derives.
//! Error types follow the hierarchy:
//! - Parse/Search/Watcher -> IndexError -> CommandError
//! - Model -> Retrieval/Daemon/DocGen -> CommandError

pub mod claude;
pub mod command;
pub mod daemon;
pub mod docgen;
pub mod index;
pub mod memory;
pub mod model;
pub mod parse;
pub mod retrieval;
pub mod search;
pub mod watcher;

pub use claude::{ClaudeError, ClaudeResult};
pub use command::{CommandError, CommandResult};
pub use daemon::DaemonError;
pub use docgen::DocGenError;
pub use index::{IndexError, IndexManagerResult};
pub use memory::{MemoryError, MemoryResult};
pub use model::{ModelError, ModelResult};
pub use parse::ParseError;
pub use retrieval::{RetrievalError, RetrievalResult};
pub use search::{SearchError, SearchResult};
pub use watcher::{WatcherError, WatcherResult};
