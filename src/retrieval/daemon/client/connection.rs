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

/// Configure timeouts on a connected stream
fn configure_stream(
	stream: &UnixStream,
	timeout: Duration,
) {
	stream.set_read_timeout(Some(timeout)).ok();
	stream.set_write_timeout(Some(timeout)).ok();
}

/// Connect to the daemon, auto-starting if needed
pub fn connect(
	socket_path: &Path,
	timeout: Duration,
) -> RetrievalResult<UnixStream> {
	if let Ok(stream) = UnixStream::connect(socket_path) {
		configure_stream(&stream, timeout);
		return Ok(stream);
	}
	// daemon not running, try to start it
	start_daemon(socket_path)?;
	std::thread::sleep(Duration::from_millis(500));
	let stream = UnixStream::connect(socket_path)
		.map_err(|err| {
			RetrievalError::DaemonNotRunning(format!(
				"failed to connect after start: {}",
				err
			))
		})?;
	configure_stream(&stream, timeout);
	Ok(stream)
}

/// Send a request once without retry logic
pub fn send_request_once(
	socket_path: &Path,
	timeout: Duration,
	request: &DaemonRequest,
) -> RetrievalResult<DaemonResponse> {
	let mut stream = connect(socket_path, timeout)?;
	let bytes = serialize_request(request).map_err(
		|err| {
			let msg = format!("serialize: {}", err);
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
		.map_err(|err| {
			let msg = format!("deserialize: {}", err);
			RetrievalError::DaemonCommunication(msg)
		})
}

/// Apply exponential backoff between retries
fn apply_backoff(delay_ms: &mut u64, config: &RetryConfig) {
	let multiplied =
		((*delay_ms as f32) * config.backoff_multiplier) as u64;
	*delay_ms = multiplied.min(config.max_delay_ms);
}

/// Handle a single retry attempt, logging on failure
#[allow(clippy::print_stderr)]
fn handle_retry_attempt(
	attempt: usize,
	delay_ms: &mut u64,
	config: &RetryConfig,
) {
	if attempt < config.max_retries {
		eprintln!(
			"[daemon-client] Retry {}/{} after {}ms",
			attempt + 1,
			config.max_retries,
			delay_ms
		);
		std::thread::sleep(
			std::time::Duration::from_millis(*delay_ms),
		);
		apply_backoff(delay_ms, config);
	}
}

/// Send a request with retry logic and exponential backoff
pub fn send_request(
	socket_path: &Path,
	timeout: Duration,
	request: &DaemonRequest,
) -> RetrievalResult<DaemonResponse> {
	let config = RetryConfig::default();
	let mut delay_ms = config.initial_delay_ms;
	let mut last_err: Option<RetrievalError> = None;

	for attempt in 0..=config.max_retries {
		match send_request_once(
			socket_path, timeout, request,
		) {
			Ok(response) => return Ok(response),
			Err(err) => {
				if !is_retryable_error(&err) {
					return Err(err);
				}
				last_err = Some(err);
				handle_retry_attempt(
					attempt, &mut delay_ms, &config,
				);
			}
		}
	}

	let fallback = RetrievalError::DaemonCommunication(
		"max retries exceeded".to_string(),
	);
	Err(last_err.unwrap_or(fallback))
}
