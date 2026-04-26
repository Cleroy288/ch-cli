mod claude;
mod claude_args;
mod claude_parse;
mod detect;
mod factory;
mod gemini;
mod traits;

pub use claude::ClaudeBackend;
pub use detect::{detect_backends, is_installed};
pub use factory::backend_for_kind;
pub use gemini::GeminiBackend;
pub use traits::CliBackend;
