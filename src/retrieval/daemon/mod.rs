//! Model Daemon Module
//!
//! Background daemon for ML models.
//!
//! Components:
//! - `protocol`: IPC message types
//! - `protocol_types`: Shared data structs
//! - `server`: Background model server
//! - `client`: Client to communicate
//! - `lifecycle`: Start/stop/status
//! - `health`: Health checks and recovery

mod async_methods;
mod async_ops;
mod health_check;
#[doc(hidden)]
pub mod lifecycle_helpers;
mod lifecycle_pid;
mod protocol_serde;

pub mod async_client;
pub mod client;
pub mod health;
pub mod lifecycle;
pub mod protocol;
pub mod protocol_types;
pub mod server;

pub use async_client::AsyncDaemonClient;
pub use client::{DaemonClient, DocGenStatus};
pub use health::{
	ensure_daemon_ready, ensure_healthy_daemon,
	health_check, prewarm_daemon, HealthStatus,
};
pub use lifecycle::{
	daemon_status, default_paths, log_file_from_socket,
	restart_daemon, signal_daemon_stop, start_daemon,
	stop_daemon,
};
pub use protocol::{
	DaemonRequest, DaemonResponse, DaemonStatus,
	DocEntryResponse,
};
pub use server::{DocGenProgress, ModelDaemon};
