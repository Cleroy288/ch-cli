//! Chunk extraction from markdown content.
//!
//! Processes markdown lines into DocChunks using
//! header detection and content accumulation.

use super::chunk::DocChunk;

/// Mutable state for chunk extraction
#[derive(Default)]
pub struct ChunkState {
	/// collected finished chunks
	pub chunks: Vec<DocChunk>,
	/// chunk currently being built
	pub current_chunk: Option<DocChunk>,
	/// hierarchy stack: (level, title)
	pub parent_stack: Vec<(usize, String)>,
	/// accumulated content for current chunk
	pub content_buffer: String,
}

/// Process a single markdown line
pub fn process_line(
	line: &str,
	line_num: usize,
	state: &mut ChunkState,
) {
	if let Some((level, title)) = parse_header(line)
	{
		start_header(level, title, line_num, state);
	} else if state.current_chunk.is_some() {
		state.content_buffer.push_str(line);
		state.content_buffer.push('\n');
	} else if !line.trim().is_empty() {
		state.current_chunk = Some(DocChunk {
			title: "Introduction".to_string(),
			content: String::new(),
			level: 0,
			line: line_num,
			parent: None,
		});
		state.content_buffer.push_str(line);
		state.content_buffer.push('\n');
	}
}

/// Parse a markdown header line (# Title)
pub fn parse_header(
	line: &str,
) -> Option<(usize, String)> {
	let trimmed = line.trim();
	if !trimmed.starts_with('#') {
		return None;
	}
	let level = trimmed
		.chars()
		.take_while(|&chr| chr == '#')
		.count();
	if level > 6 {
		return None;
	}
	let title = trimmed[level..]
		.trim()
		.trim_end_matches('#')
		.trim();
	if title.is_empty() {
		return None;
	}
	Some((level, title.to_string()))
}

/// Handle a header line: flush previous, start new
fn start_header(
	level: usize,
	title: String,
	line_num: usize,
	state: &mut ChunkState,
) {
	flush_chunk(state);
	state.content_buffer.clear();

	// update parent stack
	while state
		.parent_stack
		.last()
		.is_some_and(|(lvl, _)| *lvl >= level)
	{
		state.parent_stack.pop();
	}

	let parent = state
		.parent_stack
		.last()
		.map(|(_, name)| name.clone());

	state.current_chunk = Some(DocChunk {
		title: title.clone(),
		content: String::new(),
		level,
		line: line_num,
		parent,
	});
	state.parent_stack.push((level, title));
}

/// Flush current chunk into the chunks vec
pub fn flush_chunk(state: &mut ChunkState) {
	let Some(mut chunk) =
		state.current_chunk.take()
	else {
		return;
	};
	chunk.content =
		state.content_buffer.trim().to_string();
	if !chunk.content.is_empty()
		|| !chunk.title.is_empty()
	{
		state.chunks.push(chunk);
	}
}
