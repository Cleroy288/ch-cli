pub mod cli_args;
pub mod enhance;
pub mod json_parse;
pub mod mode_prompt;
pub mod mode_prompts_text;
pub mod model;
pub mod process;
pub mod session;
pub mod process_stream;
pub mod prompt;
pub mod stream_parse;
mod stream_parse_extract;
pub mod stream_tool_parse;

pub use json_parse::{
	parse_claude_response, parse_response_permissive,
};
pub use mode_prompt::{
	default_instruction, detect_query_mode,
	mode_instruction, wrap_prompt,
};
pub use model::{
	is_valid_model, load_effort, load_model,
	save_effort, save_model,
};
pub use process::execute_claude_cli;
pub use process_stream::spawn_streaming;
pub use prompt::build_prompt;
pub use stream_parse::parse_stream_line;
