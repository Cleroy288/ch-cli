//! Protocol Serialization/Deserialization
//!
//! Extracted from protocol.rs for norm compliance.

use super::protocol::{DaemonRequest, DaemonResponse};

/// Serialize a request to JSON bytes with newline delimiter
pub fn serialize_request(
	request: &DaemonRequest,
) -> Result<Vec<u8>, serde_json::Error> {
	let mut bytes = serde_json::to_vec(request)?;
	bytes.push(b'\n'); // delimiter for line-based protocol
	Ok(bytes)
}

/// Deserialize a request from JSON bytes
pub fn deserialize_request(
	bytes: &[u8],
) -> Result<DaemonRequest, serde_json::Error> {
	serde_json::from_slice(bytes)
}

/// Serialize a response to JSON bytes with newline
pub fn serialize_response(
	response: &DaemonResponse,
) -> Result<Vec<u8>, serde_json::Error> {
	let mut bytes = serde_json::to_vec(response)?;
	bytes.push(b'\n'); // delimiter for line-based protocol
	Ok(bytes)
}

/// Deserialize a response from JSON bytes
pub fn deserialize_response(
	bytes: &[u8],
) -> Result<DaemonResponse, serde_json::Error> {
	serde_json::from_slice(bytes)
}
