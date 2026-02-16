//! Helper functions for DefaultDaemonService.

use crate::domain::errors::daemon::DaemonError;
use crate::retrieval::daemon::DaemonClient;

use super::types::DaemonStatusInfo;

/// Build status info from a running daemon
pub fn build_status(
	pid: u32,
) -> Result<DaemonStatusInfo, DaemonError> {
	let client = DaemonClient::new();
	match client.status() {
		Ok(status) => Ok(DaemonStatusInfo {
			is_running: true,
			pid: Some(pid),
			loaded_models: status.loaded_models,
			device: Some(
				status.device.device_type,
			),
			device_detail: Some(
				status.device.device_name,
			),
			gpu_memory_mb: status.device.memory_mb,
			uptime_secs: Some(status.uptime_secs),
			is_reachable: true,
			error: None,
		}),
		Err(err) => Ok(DaemonStatusInfo {
			is_running: true,
			pid: Some(pid),
			is_reachable: false,
			error: Some(err.to_string()),
			..DaemonStatusInfo::default()
		}),
	}
}
