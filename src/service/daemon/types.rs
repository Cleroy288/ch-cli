//! DTOs for the daemon service.

/// Daemon status information
#[derive(Debug, Clone)]
pub struct DaemonStatusInfo {
	/// Whether the daemon is running
	pub is_running: bool,
	/// Process ID if running
	pub pid: Option<u32>,
	/// Loaded model names
	pub loaded_models: Vec<String>,
	/// Device type (CPU/GPU)
	pub device: Option<String>,
	/// Device details
	pub device_detail: Option<String>,
	/// GPU memory in MB (if applicable)
	pub gpu_memory_mb: Option<u64>,
	/// Uptime in seconds
	pub uptime_secs: Option<u64>,
	/// Whether daemon is reachable
	pub is_reachable: bool,
	/// Error message if unreachable
	pub error: Option<String>,
}

impl Default for DaemonStatusInfo {
	fn default() -> Self {
		Self {
			is_running: false,
			pid: None,
			loaded_models: vec![],
			device: None,
			device_detail: None,
			gpu_memory_mb: None,
			uptime_secs: None,
			is_reachable: false,
			error: None,
		}
	}
}
