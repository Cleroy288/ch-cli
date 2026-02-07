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
		Ok(s) => Ok(DaemonStatusInfo {
			is_running: true,
			pid: Some(pid),
			loaded_models: s.loaded_models,
			device: Some(s.device.device_type),
			device_detail: Some(
				s.device.device_name,
			),
			gpu_memory_mb: s.device.memory_mb,
			uptime_secs: Some(s.uptime_secs),
			is_reachable: true,
			error: None,
		}),
		Err(e) => Ok(DaemonStatusInfo {
			is_running: true,
			pid: Some(pid),
			is_reachable: false,
			error: Some(e.to_string()),
			..DaemonStatusInfo::default()
		}),
	}
}
