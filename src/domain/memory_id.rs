//! ID and timestamp generation for memory types.

use std::sync::atomic::{
	AtomicU32, Ordering,
};
use std::time::{SystemTime, UNIX_EPOCH};

use super::memory::{
	AiResponse, Interaction, UserInput,
};

/// Monotonic counter to guarantee uniqueness
static COUNTER: AtomicU32 = AtomicU32::new(0);

/// Generate a unique interaction ID
pub fn generate_id() -> String {
	let now = current_timestamp();
	let rand: u32 = random_u32();
	format!("{now}-{rand:08x}")
}

/// Current time as unix epoch seconds
pub fn current_timestamp() -> u64 {
	SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap_or_default()
		.as_secs()
}

/// Generate a new session ID (date + random)
pub fn new_session_id() -> String {
	let now = current_timestamp();
	let rand: u32 = random_u32();
	format!("s-{now}-{rand:04x}")
}

/// Build a new Interaction from components
pub fn new_interaction(
	session_id: &str,
	input: UserInput,
	response: AiResponse,
) -> Interaction {
	Interaction {
		id: generate_id(),
		timestamp: current_timestamp(),
		session_id: session_id.to_string(),
		input,
		response,
	}
}

/// Simple unique u32 using timestamp + pid + counter
fn random_u32() -> u32 {
	let nanos = SystemTime::now()
		.duration_since(UNIX_EPOCH)
		.unwrap_or_default()
		.as_nanos();
	let pid = std::process::id();
	let seq = COUNTER.fetch_add(1, Ordering::Relaxed);
	(nanos as u32)
		.wrapping_mul(pid)
		.wrapping_add((nanos >> 32) as u32)
		.wrapping_add(seq)
}
