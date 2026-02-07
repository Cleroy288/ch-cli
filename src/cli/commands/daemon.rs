//! Daemon management commands.
//!
//! Thin handlers that delegate to DaemonService
//! for all lifecycle operations.

use crate::service::{
	DaemonService, DefaultDaemonService,
};

use super::error::CommandResult;

/// Execute the `daemon start` command
pub fn daemon_start_command() -> CommandResult {
	let svc = DefaultDaemonService::new();
	let to_io = |e| {
		std::io::Error::new(
			std::io::ErrorKind::Other,
			format!("{}", e),
		)
	};

	let status = svc.status().map_err(to_io)?;
	if status.is_running {
		println!(
			"Daemon is already running (PID: {})",
			status.pid.unwrap_or(0)
		);
		return Ok(());
	}

	println!("Starting model daemon...");
	svc.start().map_err(to_io)?;

	std::thread::sleep(
		std::time::Duration::from_millis(500),
	);

	let status = svc.status().map_err(to_io)?;
	if status.is_running {
		println!(
			"Daemon started (PID: {})",
			status.pid.unwrap_or(0)
		);
		println!("Models loading in background...");
	} else {
		println!(
			"Warning: Daemon may not have started"
		);
	}
	Ok(())
}

/// Execute the `daemon stop` command
pub fn daemon_stop_command() -> CommandResult {
	let svc = DefaultDaemonService::new();
	let to_io = |e| {
		std::io::Error::new(
			std::io::ErrorKind::Other,
			format!("{}", e),
		)
	};

	let status = svc.status().map_err(to_io)?;
	if !status.is_running {
		println!("Daemon is not running");
		return Ok(());
	}

	let pid = status.pid.unwrap_or(0);
	println!("Stopping daemon (PID: {})...", pid);
	svc.stop().map_err(to_io)?;
	println!("Daemon stopped");
	Ok(())
}

/// Execute the `daemon status` command
pub fn daemon_status_command() -> CommandResult {
	let svc = DefaultDaemonService::new();
	let err = |e| std::io::Error::new(
		std::io::ErrorKind::Other, format!("{e}"),
	);
	let s = svc.status().map_err(err)?;
	if !s.is_running {
		println!("Daemon Status: Not running");
		return Ok(());
	}
	if !s.is_reachable {
		println!("Daemon Status: Running (unreachable)");
		let pid = s.pid.unwrap_or(0);
		println!("PID:           {}", pid);
		if let Some(ref e) = s.error {
			println!("Error:         {}", e);
		}
		return Ok(());
	}
	let pid = s.pid.unwrap_or(0);
	println!("Daemon Status: Running");
	println!("PID:           {}", pid);
	if let (Some(d), Some(dd)) =
		(&s.device, &s.device_detail)
	{
		println!("Device:        {} ({})", d, dd);
	}
	if let Some(mem) = s.gpu_memory_mb {
		println!("GPU Memory:    {} MB", mem);
	}
	if let Some(up) = s.uptime_secs {
		println!("Uptime:        {} seconds", up);
	}
	println!("\nLoaded Models:");
	for model in &s.loaded_models {
		println!("  - {}", model);
	}
	if s.loaded_models.is_empty() {
		println!("  (none)");
	}
	Ok(())
}

/// Execute the `daemon restart` command
pub fn daemon_restart_command() -> CommandResult {
	daemon_stop_command()?;
	std::thread::sleep(
		std::time::Duration::from_millis(200),
	);
	daemon_start_command()
}

/// Execute the `daemon run` command (foreground)
pub fn daemon_run_command(
	socket: Option<&str>,
) -> CommandResult {
	let svc = DefaultDaemonService::new();
	svc.run_foreground(socket).map_err(|e| {
		std::io::Error::new(
			std::io::ErrorKind::Other,
			format!("{}", e),
		)
	})?;
	Ok(())
}
