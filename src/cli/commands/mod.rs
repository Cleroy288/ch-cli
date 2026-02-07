//! Command implementations for the CLI.

mod daemon;
mod docs;
mod embed;
#[doc(hidden)]
pub mod error;
mod index;
#[doc(hidden)]
pub mod info;
mod navigation;
mod retrieve;
#[doc(hidden)]
pub mod search;
#[doc(hidden)]
pub mod search_callers;
mod search_helpers;
mod search_semantic;
mod stats;

// Re-export all public types for backward compatibility
pub use daemon::{
	daemon_restart_command,
	daemon_run_command,
	daemon_start_command,
	daemon_status_command,
	daemon_stop_command,
};
pub use docs::{
	docs_generate_command,
	docs_search_command,
	docs_show_command,
	docs_status_command,
};
pub use embed::embed_command;
pub use error::{CommandError, CommandResult};
pub use index::index_command;
pub use info::info_command;
pub use navigation::{goto_command, refs_command, symbols_command};
pub use retrieve::retrieve_command;
pub use search::search_command;
pub use stats::stats_command;
