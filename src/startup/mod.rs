mod action;
pub mod agent_discovery;
pub mod agent_select;
mod boot;
mod checks;
pub mod credentials;
mod credentials_collect;
mod key_reader;
pub mod mcp_registration;
mod mcp_registration_io;
pub mod migration;
mod migration_legacy;
pub mod migration_config;
mod migration_config_loaders;
mod migration_config_readers;
pub mod integration_mode;
mod progress;
mod progress_draw;
pub mod repo_scan;
mod repo_scan_display;
mod progress_incremental;
mod progress_incremental_draw;
mod progress_shared;
mod progress_spawn;
mod prompts;
mod prompts_analysis;
mod prompts_analysis_display;
mod prompts_index;
mod prompts_shared;
mod unsupported_display;
pub mod watcher;
mod watcher_handle;
mod watcher_loop;

pub use action::StartupAction;
pub(crate) use boot::mark_setup_done;
pub use boot::run_startup;
pub use checks::{
	analyze_codebase, check_index_exists,
	detect_codebase_changes,
};
pub use mcp_registration::ensure_mcp_registered;
pub use progress::index_with_progress;
pub use progress_incremental
	::index_with_progress_incremental;
pub use prompts::prompt_for_update;
pub use prompts_analysis
	::prompt_for_indexing_with_analysis;
pub use prompts_index::prompt_for_indexing;
pub use agent_discovery::start_mcp_discovery;
pub use agent_select::check_and_prompt_agent;
pub use credentials::check_and_prompt_credentials;
pub use watcher::{spawn_watcher, WatcherHandle};
