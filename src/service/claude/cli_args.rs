use crate::domain::errors::ClaudeError;

pub const CLAUDE_BIN: &str = "claude";
pub const FLAG_PRINT: &str = "-p";
pub const FLAG_OUTPUT_FMT: &str = "--output-format";
pub const FMT_JSON: &str = "json";
pub const FMT_STREAM: &str = "stream-json";
pub const FLAG_RESUME: &str = "--resume";
pub const FLAG_SKIP_PERMS: &str =
	"--dangerously-skip-permissions";
pub const FLAG_MODEL: &str = "--model";
pub const FLAG_VERBOSE: &str = "--verbose";
pub const FLAG_PARTIAL: &str =
	"--include-partial-messages";
pub const FLAG_APPEND_SYS: &str =
	"--append-system-prompt";
pub const FLAG_EFFORT: &str = "--effort";

pub fn map_io_error(
	err: std::io::Error,
) -> ClaudeError {
	match err.kind() {
		std::io::ErrorKind::NotFound => {
			ClaudeError::NotInstalled
		}
		_ => ClaudeError::Process(
			err.to_string(),
		),
	}
}
