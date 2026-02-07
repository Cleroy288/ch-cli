//! Connection Management
//!
//! Handles connection establishment and request sending to the daemon.

use std::io::{BufRead, BufReader, Write};
use std::os::unix::net::UnixStream;
use std::path::Path;
use std::time::Duration;

use super::retry::{is_retryable_error, RetryConfig};
use crate::retrieval::daemon::lifecycle::start_daemon;
use crate::retrieval::daemon::protocol::{
	deserialize_response, serialize_request, DaemonRequest, DaemonResponse,
};
use crate::retrieval::{RetrievalError, RetrievalResult};

/// Connect to the daemon, auto-starting if needed
pub fn connect(
	socket_path: &Path,
	timeout: Duration,
) -> RetrievalResult<UnixStream> {
	// try to connect first
	match UnixStream::connect(socket_path) {
		Ok(stream) => {
			stream.set_read_timeout(Some(timeout)).ok();
			stream.set_write_timeout(Some(timeout)).ok();
			Ok(stream)
		}
		Err(_) => {
			// daemon not running, try to start it
			start_daemon(socket_path)?;
			// wait a bit for daemon to start
			std::thread::sleep(Duration::from_millis(500));
			// try again
			let stream = UnixStream::connect(socket_path).map_err(|e| {
				RetrievalError::DaemonNotRunning(format!(
					"failed to connect after start: {}",
					e
				))
			})?;
			stream.set_read_timeout(Some(timeout)).ok();
			stream.set_write_timeout(Some(timeout)).ok();
			Ok(stream)
		}
	}
}

/// Send a request once without retry logic
pub fn send_request_once(
	socket_path: &Path,
	timeout: Duration,
	request: &DaemonRequest,
) -> RetrievalResult<DaemonResponse> {
	let mut stream = connect(socket_path, timeout)?;
	let bytes = serialize_request(request).map_err(
		|e| {
			let msg = format!("serialize: {}", e);
			RetrievalError::DaemonCommunication(msg)
		},
	)?;

	stream.write_all(&bytes)?;
	stream.flush()?;

	// read response line
	let mut reader = BufReader::new(stream); // buffered reader for response
	let mut line = String::new(); // response line buffer
	reader.read_line(&mut line)?;

	deserialize_response(line.trim().as_bytes())
		.map_err(|e| {
			let msg = format!("deserialize: {}", e);
			RetrievalError::DaemonCommunication(msg)
		})
}

/// Send a request with retry logic and exponential backoff
pub fn send_request(
	socket_path: &Path,
	timeout: Duration,
	request: &DaemonRequest,
) -> RetrievalResult<DaemonResponse> {
	let config = RetryConfig::default(); // retry configuration
	let mut delay_ms = config.initial_delay_ms; // current delay between retries
	let mut last_error: Option<RetrievalError> = None; // last error encountered

	for attempt in 0..=config.max_retries {
		match send_request_once(socket_path, timeout, request) {
			Ok(response) => return Ok(response),
			Err(e) => {
				// Check if error is retryable
				if !is_retryable_error(&e) {
					return Err(e);
				}

				last_error = Some(e);

				if attempt < config.max_retries {
					eprintln!(
						"[daemon-client] Retry {}/{} after {}ms",
						attempt + 1,
						config.max_retries,
						delay_ms
					);
					std::thread::sleep(std::time::Duration::from_millis(delay_ms));

					// Exponential backoff with max cap
					delay_ms = ((delay_ms as f32) * config.backoff_multiplier) as u64;
					delay_ms = delay_ms.min(config.max_delay_ms);
				}
			}
		}
	}

	Err(last_error.unwrap_or_else(|| {
		RetrievalError::DaemonCommunication("max retries exceeded".to_string())
	}))
}
