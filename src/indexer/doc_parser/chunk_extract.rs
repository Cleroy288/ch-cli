use super::chunk::DocChunk;

#[derive(Default)]
pub struct ChunkState {
	pub chunks: Vec<DocChunk>,
	pub current_chunk: Option<DocChunk>,
	/// (level, title)
	pub parent_stack: Vec<(usize, String)>,
	pub content_buffer: String,
}

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
