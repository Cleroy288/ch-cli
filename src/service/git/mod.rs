mod discover;
mod graph_build;
mod graph_builder;
mod graph_connect;
mod log_parse;
mod log_parse_fields;
mod watch_handle;
mod watch_mtime;
pub mod watcher;

pub use discover::discover_git_repos;
pub use graph_build::build_graph;
pub use log_parse::fetch_git_log;
pub use log_parse_fields::{parse_parents, parse_refs};
pub use watch_handle::GitWatchHandle;
