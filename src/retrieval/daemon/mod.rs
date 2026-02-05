//! Model Daemon Module
//!
//! Provides a background daemon that keeps ML models loaded in memory
//! to avoid reload latency on each request.
//!
//! Components:
//! - `protocol`: IPC message types (DaemonRequest, DaemonResponse)
//! - `server`: Background model server
//! - `client`: Client to communicate with daemon
//! - `lifecycle`: Start/stop/status management

pub mod async_client;
pub mod client;
pub mod lifecycle;
pub mod protocol;
pub mod server;

pub use async_client::AsyncDaemonClient;
pub use client::{DaemonClient, DocGenStatus};
pub use lifecycle::{daemon_status, ensure_daemon_ready, prewarm_daemon, start_daemon, stop_daemon};
pub use protocol::{DaemonRequest, DaemonResponse, DaemonStatus, DocEntryResponse};
pub use server::ModelDaemon;
