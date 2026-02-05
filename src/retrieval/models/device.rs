//! GPU Device Detection and Management
//!
//! Provides runtime GPU detection with graceful fallback to CPU.
//! Supports Metal (macOS) and CUDA (Linux/Windows).

use candle_core::Device;
use std::env;
use std::fmt;

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
	let val = env::var("CH_FORCE_CPU").unwrap_or_default(); // env var value or empty
	val == "1" || val.to_lowercase() == "true"
}

/// Detect available GPU device at runtime
pub fn detect_device() -> DeviceInfo {
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

/// Get the best available device with automatic fallback
pub fn get_device_with_fallback() -> Device {
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

/// Get Metal device name (macOS only)
#[cfg(feature = "metal")]
fn get_metal_device_name() -> String {
	// metal-rs could provide device name but for simplicity use sysctl
	#[cfg(target_os = "macos")]
	{
		use std::process::Command;
		let output = Command::new("sysctl")
			.args(["-n", "machdep.cpu.brand_string"])
			.output();
		if let Ok(out) = output {
			let cpu = String::from_utf8_lossy(&out.stdout); // CPU brand string
			if cpu.contains("Apple") {
				return "Apple Silicon GPU".to_string();
			}
		}
	}
	"Metal GPU".to_string()
}

/// Get Metal GPU memory (macOS only)
#[cfg(feature = "metal")]
fn get_metal_memory_mb() -> Option<u64> {
	#[cfg(target_os = "macos")]
	{
		use std::process::Command;
		let output = Command::new("sysctl")
			.args(["-n", "hw.memsize"])
			.output();
		if let Ok(out) = output {
			let mem_str = String::from_utf8_lossy(&out.stdout); // total system memory
			if let Ok(bytes) = mem_str.trim().parse::<u64>() {
				// unified memory: report total system memory
				return Some(bytes / 1_000_000);
			}
		}
	}
	None
}

/// Get CUDA device name (placeholder)
#[cfg(feature = "cuda")]
fn get_cuda_device_name() -> String {
	"NVIDIA GPU".to_string()
}

/// Get CUDA GPU memory (placeholder)
#[cfg(feature = "cuda")]
fn get_cuda_memory_mb() -> Option<u64> {
	None
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_device_detection() {
		let info = detect_device(); // get current device info
		assert!(!info.device_name.is_empty());
		// device_type should be one of the valid variants
		match info.device_type {
			DeviceType::Cpu | DeviceType::Metal | DeviceType::Cuda => {}
		}
	}

	#[test]
	fn test_force_cpu_env_var() {
		// save original value
		let original = env::var("CH_FORCE_CPU").ok();

		// test with "1"
		env::set_var("CH_FORCE_CPU", "1");
		assert!(is_force_cpu_mode());

		// test with "true"
		env::set_var("CH_FORCE_CPU", "true");
		assert!(is_force_cpu_mode());

		// test with "TRUE"
		env::set_var("CH_FORCE_CPU", "TRUE");
		assert!(is_force_cpu_mode());

		// test with empty/unset
		env::remove_var("CH_FORCE_CPU");
		assert!(!is_force_cpu_mode());

		// restore original value
		match original {
			Some(val) => env::set_var("CH_FORCE_CPU", val),
			None => env::remove_var("CH_FORCE_CPU"),
		}
	}

	#[test]
	fn test_fallback_to_cpu() {
		// save original value
		let original = env::var("CH_FORCE_CPU").ok();

		// force CPU mode
		env::set_var("CH_FORCE_CPU", "1");
		let device = get_device_with_fallback(); // should return CPU

		// verify it's CPU (Device doesn't impl PartialEq so check via debug)
		let debug_str = format!("{:?}", device);
		assert!(debug_str.contains("Cpu"));

		// restore original value
		match original {
			Some(val) => env::set_var("CH_FORCE_CPU", val),
			None => env::remove_var("CH_FORCE_CPU"),
		}
	}

	#[test]
	fn test_device_type_display() {
		assert_eq!(format!("{}", DeviceType::Cpu), "CPU");
		assert_eq!(format!("{}", DeviceType::Metal), "Metal");
		assert_eq!(format!("{}", DeviceType::Cuda), "CUDA");
	}
}
