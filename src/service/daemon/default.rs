//! Default implementation of DaemonService.

use crate::domain::errors::daemon::DaemonError;
use crate::retrieval::daemon::{
	daemon_status, start_daemon, stop_daemon,
	ModelDaemon,
};
use crate::retrieval::RetrievalConfig;

use super::helpers::build_status;
use super::types::DaemonStatusInfo;
use super::DaemonService;

/// Default daemon service backed by DaemonClient
pub struct DefaultDaemonService;

impl Default for DefaultDaemonService {
	fn default() -> Self {
		Self
	}
}

impl DefaultDaemonService {
	/// Create a new default daemon service
	pub fn new() -> Self {
		Self
	}
}

impl DaemonService for DefaultDaemonService {
	fn start(&self) -> Result<(), DaemonError> {
		let config = RetrievalConfig::default();
		start_daemon(&config.socket_path)
			.map_err(|err| {
				DaemonError::StartFailed(
					err.to_string(),
				)
			})
	}

	fn stop(&self) -> Result<(), DaemonError> {
		let config = RetrievalConfig::default();
		stop_daemon(&config.socket_path)
			.map_err(|err| {
				DaemonError::StopFailed(
					err.to_string(),
				)
			})
	}

	fn restart(&self) -> Result<(), DaemonError> {
		self.stop().ok();
		std::thread::sleep(
			std::time::Duration::from_millis(500),
		);
		self.start()
	}

	fn status(
		&self,
	) -> Result<DaemonStatusInfo, DaemonError> {
		let config = RetrievalConfig::default();
		let status =
			daemon_status(&config.socket_path);
		if !status.running {
			return Ok(
				DaemonStatusInfo::default(),
			);
		}
		let pid = status.pid.unwrap_or(0);
		build_status(pid)
	}

	fn run_foreground(
		&self,
		socket: Option<&str>,
	) -> Result<(), DaemonError> {
		let mut daemon = match socket {
			Some(path) => {
				ModelDaemon::with_socket_path(path)
			}
			None => ModelDaemon::new(),
		};
		daemon.run().map_err(|err| {
			DaemonError::StartFailed(err.to_string())
		})
	}
}
