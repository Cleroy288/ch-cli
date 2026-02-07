//! GPU Device Detection and Management
//!
//! Provides runtime GPU detection with graceful fallback to CPU.
//! Supports Metal (macOS) and CUDA (Linux/Windows).

use candle_core::Device;
use std::env;
use std::fmt;

#[cfg(feature = "metal")]
use super::device_platform::{
	get_metal_device_name, get_metal_memory_mb,
};
#[cfg(feature = "cuda")]
use super::device_platform::{
	get_cuda_device_name, get_cuda_memory_mb,
};

/// Device type enumeration for GPU/CPU backends
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceType {
	/// CPU computation (fallback)
	Cpu,
	/// Apple Metal GPU (macOS)
	Metal,
	/// NVIDIA CUDA GPU
	Cuda,
}

impl fmt::Display for DeviceType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			DeviceType::Cpu => write!(f, "CPU"),
			DeviceType::Metal => write!(f, "Metal"),
			DeviceType::Cuda => write!(f, "CUDA"),
		}
	}
}

/// Information about the detected compute device
#[derive(Debug, Clone)]
pub struct DeviceInfo {
	/// type of device (CPU, Metal, CUDA)
	pub device_type: DeviceType,
	/// human-readable device name
	pub device_name: String,
	/// GPU memory in megabytes (None for CPU)
	pub memory_mb: Option<u64>,
}

impl Default for DeviceInfo {
	fn default() -> Self {
		Self {
			device_type: DeviceType::Cpu,
			device_name: "CPU".to_string(),
			memory_mb: None,
		}
	}
}

/// Check if CPU mode is forced via environment variable
pub fn is_force_cpu_mode() -> bool {
	// env var value or empty string
	let val = env::var("CH_FORCE_CPU").unwrap_or_default();
	val == "1" || val.to_lowercase() == "true"
}

/// Get information about the detected compute device
pub fn get_device_info() -> DeviceInfo {
	// check force CPU mode first
	if is_force_cpu_mode() {
		return DeviceInfo {
			device_type: DeviceType::Cpu,
			device_name: "CPU (forced via CH_FORCE_CPU)".to_string(),
			memory_mb: None,
		};
	}

	// try Metal on macOS
	#[cfg(feature = "metal")]
	{
		if let Ok(_device) = Device::new_metal(0) {
			return DeviceInfo {
				device_type: DeviceType::Metal,
				device_name: get_metal_device_name(),
				memory_mb: get_metal_memory_mb(),
			};
		}
	}

	// try CUDA on Linux/Windows
	#[cfg(feature = "cuda")]
	{
		if Device::cuda_if_available(0).is_ok() {
			return DeviceInfo {
				device_type: DeviceType::Cuda,
				device_name: get_cuda_device_name(),
				memory_mb: get_cuda_memory_mb(),
			};
		}
	}

	// fallback to CPU
	DeviceInfo::default()
}

/// Get the default device with runtime detection and fallback
pub fn get_device() -> Device {
	// check force CPU mode first
	if is_force_cpu_mode() {
		return Device::Cpu;
	}

	// try Metal on macOS
	#[cfg(feature = "metal")]
	{
		if let Ok(device) = Device::new_metal(0) {
			return device;
		}
	}

	// try CUDA on Linux/Windows
	#[cfg(feature = "cuda")]
	{
		if let Ok(device) = Device::cuda_if_available(0) {
			return device;
		}
	}

	// fallback to CPU
	Device::Cpu
}

