use std::env;

use ch_cli::retrieval::models::device::{
	get_device, get_device_info, is_force_cpu_mode,
	DeviceType,
};

#[test]
fn test_device_detection() {
	let info = get_device_info(); // get current device info
	assert!(!info.device_name.is_empty());
	// device_type should be one of the valid variants
	match info.device_type {
		DeviceType::Cpu
		| DeviceType::Metal
		| DeviceType::Cuda => {}
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
	let device = get_device(); // should return CPU

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
