//! Daemon management commands.
//!
//! Thin handlers that delegate to DaemonService
//! for all lifecycle operations.

use std::io::Write;

use crate::retrieval::daemon::log_file_from_socket;
use crate::retrieval::RetrievalConfig;
use crate::service::daemon::types::DaemonStatusInfo;
use crate::service::{
	DaemonService, DefaultDaemonService,
};

use super::error::CommandResult;

/// Convert any Display error to io::Error
fn to_io_error(
	err: impl std::fmt::Display,
) -> std::io::Error {
	std::io::Error::other(format!("{}", err))
}

/// Execute the `daemon start` command
pub fn daemon_start_command() -> CommandResult {
	let svc = DefaultDaemonService::new();
	let mut out = std::io::stdout().lock();

	let status =
		svc.status().map_err(to_io_error)?;
	if status.is_running {
		writeln!(
			out,
			"Daemon is already running (PID: {})",
			status.pid.unwrap_or(0)
		)?;
		return Ok(());
	}

	writeln!(out, "Starting model daemon...")?;
	svc.start().map_err(to_io_error)?;
	std::thread::sleep(
		std::time::Duration::from_millis(500),
	);

	let status =
		svc.status().map_err(to_io_error)?;
	print_start_result(&mut out, &status)?;
	Ok(())
}

/// Print the result of a daemon start attempt
fn print_start_result(
	out: &mut impl Write,
	status: &DaemonStatusInfo,
) -> std::io::Result<()> {
	if status.is_running {
		let config = RetrievalConfig::default();
		let log_path =
			log_file_from_socket(&config.socket_path);
		writeln!(
			out,
			"Daemon started (PID: {})",
			status.pid.unwrap_or(0)
		)?;
		writeln!(
			out, "Models loading in background..."
		)?;
		writeln!(
			out,
			"Daemon logs: {}",
			log_path.display()
		)?;
	} else {
		writeln!(
			out,
			"Warning: Daemon may not have started"
		)?;
	}
	Ok(())
}

/// Execute the `daemon stop` command
pub fn daemon_stop_command() -> CommandResult {
	let svc = DefaultDaemonService::new();
	let mut out = std::io::stdout().lock();

	let status =
		svc.status().map_err(to_io_error)?;
	if !status.is_running {
		writeln!(out, "Daemon is not running")?;
		return Ok(());
	}

	let pid = status.pid.unwrap_or(0);
	writeln!(
		out, "Stopping daemon (PID: {})...", pid
	)?;
	svc.stop().map_err(to_io_error)?;
	writeln!(out, "Daemon stopped")?;
	Ok(())
}

/// Execute the `daemon status` command
pub fn daemon_status_command() -> CommandResult {
	let svc = DefaultDaemonService::new();
	let mut out = std::io::stdout().lock();
	let status =
		svc.status().map_err(to_io_error)?;

	if !status.is_running {
		writeln!(
			out, "Daemon Status: Not running"
		)?;
		return Ok(());
	}
	if !status.is_reachable {
		print_unreachable(&mut out, &status)?;
		return Ok(());
	}
	print_running_status(&mut out, &status)?;
	Ok(())
}

/// Print status when daemon is running but
/// unreachable
fn print_unreachable(
	out: &mut impl Write,
	status: &DaemonStatusInfo,
) -> std::io::Result<()> {
	writeln!(
		out,
		"Daemon Status: Running (unreachable)"
	)?;
	let pid = status.pid.unwrap_or(0);
	writeln!(out, "PID:           {}", pid)?;
	if let Some(ref err_msg) = status.error {
		writeln!(
			out, "Error:         {}", err_msg
		)?;
	}
	Ok(())
}

/// Print full status when daemon is running
fn print_running_status(
	out: &mut impl Write,
	status: &DaemonStatusInfo,
) -> std::io::Result<()> {
	let pid = status.pid.unwrap_or(0);
	writeln!(out, "Daemon Status: Running")?;
	writeln!(out, "PID:           {}", pid)?;
	print_device_info(out, status)?;
	print_loaded_models(out, status)?;
	Ok(())
}

/// Print device and resource details
fn print_device_info(
	out: &mut impl Write,
	status: &DaemonStatusInfo,
) -> std::io::Result<()> {
	if let (Some(dev), Some(detail)) =
		(&status.device, &status.device_detail)
	{
		writeln!(
			out,
			"Device:        {} ({})",
			dev, detail
		)?;
	}
	if let Some(mem) = status.gpu_memory_mb {
		writeln!(
			out, "GPU Memory:    {} MB", mem
		)?;
	}
	if let Some(uptime) = status.uptime_secs {
		writeln!(
			out,
			"Uptime:        {} seconds",
			uptime
		)?;
	}
	Ok(())
}

/// Print the list of loaded models
fn print_loaded_models(
	out: &mut impl Write,
	status: &DaemonStatusInfo,
) -> std::io::Result<()> {
	writeln!(out, "\nLoaded Models:")?;
	if status.loaded_models.is_empty() {
		writeln!(out, "  (none)")?;
		return Ok(());
	}
	for model in &status.loaded_models {
		writeln!(out, "  - {}", model)?;
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
	svc.run_foreground(socket)
		.map_err(|err| {
			std::io::Error::other(
				format!("{}", err),
			)
		})?;
	Ok(())
}
