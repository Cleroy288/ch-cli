use std::sync::Arc;

use crate::domain::backend_kind::BackendKind;

use super::claude::ClaudeBackend;
use super::gemini::GeminiBackend;
use super::traits::CliBackend;

/// Create a backend instance from a kind.
pub fn backend_for_kind(
	kind: BackendKind,
) -> Arc<dyn CliBackend> {
	match kind {
		BackendKind::ClaudeCode => {
			Arc::new(ClaudeBackend)
		}
		BackendKind::GeminiCli => {
			Arc::new(GeminiBackend)
		}
	}
}
