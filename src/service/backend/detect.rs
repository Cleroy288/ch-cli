use std::process::Command;

use crate::domain::backend_kind::BackendKind;

/// Check if a binary exists on PATH.
fn which(binary: &str) -> bool {
	Command::new("which")
		.arg(binary)
		.output()
		.map(|o| o.status.success())
		.unwrap_or(false)
}

/// Detect installed CLI backends.
pub fn detect_backends() -> Vec<BackendKind> {
	let mut found = Vec::new();
	if which(BackendKind::ClaudeCode.binary_name()) {
		found.push(BackendKind::ClaudeCode);
	}
	if which(BackendKind::GeminiCli.binary_name()) {
		found.push(BackendKind::GeminiCli);
	}
	found
}

/// Check if a specific backend is installed.
pub fn is_installed(kind: BackendKind) -> bool {
	which(kind.binary_name())
}
