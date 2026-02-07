//! Platform-Specific Device Detection
//!
//! Functions for querying Metal GPU information on macOS.

/// Get Metal device name (macOS only)
#[cfg(feature = "metal")]
pub fn get_metal_device_name() -> String {
	#[cfg(target_os = "macos")]
	{
		use std::process::Command;
		let output = Command::new("sysctl")
			.args(["-n", "machdep.cpu.brand_string"])
			.output();
		if let Ok(out) = output {
			let cpu = String::from_utf8_lossy(&out.stdout);
			if cpu.contains("Apple") {
				return "Apple Silicon GPU".to_string();
			}
		}
	}
	"Metal GPU".to_string()
}

/// Get Metal GPU memory (macOS only)
#[cfg(feature = "metal")]
pub fn get_metal_memory_mb() -> Option<u64> {
	#[cfg(target_os = "macos")]
	{
		use std::process::Command;
		let output = Command::new("sysctl")
			.args(["-n", "hw.memsize"])
			.output();
		if let Ok(out) = output {
			let mem_str = String::from_utf8_lossy(&out.stdout);
			if let Ok(bytes) = mem_str.trim().parse::<u64>() {
				return Some(bytes / 1_000_000);
			}
		}
	}
	None
}

/// Get CUDA device name (placeholder)
#[cfg(feature = "cuda")]
pub fn get_cuda_device_name() -> String {
	"NVIDIA GPU".to_string()
}

/// Get CUDA GPU memory (placeholder)
#[cfg(feature = "cuda")]
pub fn get_cuda_memory_mb() -> Option<u64> {
	None
}
