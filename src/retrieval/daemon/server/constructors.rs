//! Server Constructors
//!
//! Extracted from mod.rs for norm compliance.

use std::path::Path;

use super::ModelDaemon;

/// Create a new daemon with default config
pub fn new_daemon() -> ModelDaemon {
	super::lifecycle::new_daemon()
}

/// Create with custom socket path
pub fn with_socket_path(
	socket_path: impl AsRef<Path>,
) -> ModelDaemon {
	super::lifecycle::with_socket_path(socket_path)
}
