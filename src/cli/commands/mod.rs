//! Command implementations for the CLI.

pub mod error;
mod index;
mod index_format;
pub mod info;
pub mod mcp_server;
pub mod memory;
pub mod memory_display;
mod navigation;
pub mod search;
pub mod search_callers;
mod search_format;
mod search_helpers;
pub mod search_strategy;
mod stats;
mod stats_format;

pub use error::{CommandError, CommandResult};
pub use index::index_command;
pub use info::{
	info_command, InfoDisplayOpts, InfoFlags,
	InfoSections,
};
pub use mcp_server::mcp_server_command;
pub use memory::{
	memory_add_command, memory_search_command,
	memory_show_command, memory_stats_command,
};
pub use navigation::{
	goto_command, refs_command, symbols_command,
};
pub use search::{
	search_command, SearchCommandFlags,
	SearchCommandOptions,
};
pub use stats::stats_command;
