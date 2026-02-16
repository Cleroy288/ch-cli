//! Lifecycle Utilities
//!
//! Extracted from lifecycle.rs for norm compliance.

use super::ModelDaemon;
use crate::retrieval::RetrievalResult;

/// Write PID file for the daemon
pub fn write_pid_file(
	daemon: &ModelDaemon,
) -> RetrievalResult<()> {
	let pid_file =
		daemon.socket_path.with_extension("pid");
	let pid = std::process::id();
	std::fs::write(&pid_file, pid.to_string())?;
	Ok(())
}
