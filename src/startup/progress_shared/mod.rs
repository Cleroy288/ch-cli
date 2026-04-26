mod format;
mod state;
mod thread;

pub(crate) use format::{
	draw_elapsed_time, format_progress_bar,
	truncate_name, SPINNER,
};
pub(crate) use state::ProgressState;
pub(crate) use thread::{
	build_progress_callback, collect_index_result,
};
