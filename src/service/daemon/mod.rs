//! Daemon service — ML daemon lifecycle
//! use cases.

mod default;
mod helpers;
pub mod types;

pub use default::DefaultDaemonService;

use crate::domain::errors::daemon::DaemonError;
use types::DaemonStatusInfo;

/// Service trait for daemon lifecycle operations
pub trait DaemonService {
	/// Start the daemon
	fn start(&self) -> Result<(), DaemonError>;

	/// Stop the daemon
	fn stop(&self) -> Result<(), DaemonError>;

	/// Restart the daemon
	fn restart(&self) -> Result<(), DaemonError>;

	/// Get daemon status
	fn status(
		&self,
	) -> Result<DaemonStatusInfo, DaemonError>;

	/// Run daemon in foreground
	fn run_foreground(
		&self,
		socket: Option<&str>,
	) -> Result<(), DaemonError>;
}
