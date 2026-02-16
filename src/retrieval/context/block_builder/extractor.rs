//! Code Extraction for BlockBuilder
//!
//! Handles extracting code snippets from source files.
//! Uses BufReader to read only the needed line range
//! instead of loading full files into memory. When byte
//! offsets are available, uses Seek for range reads.

use crate::indexer::Symbol;
use crate::retrieval::RetrievalResult;

use super::core::BlockBuilder;
use super::super::code_extractor::find_symbol_end;
use super::super::file_reader;

impl<'graph> BlockBuilder<'graph> {
	/// Extract code snippet for a symbol.
	/// Uses byte-range Seek when offsets are valid,
	/// falls back to buffered line reading otherwise.
	pub(super) fn extract_code(
		&self,
		symbol: &Symbol,
	) -> RetrievalResult<String> {
		let loc = &symbol.location;
		let before = self.config.context_lines_before;
		let after = self.config.context_lines_after;
		let has_bytes = file_reader::has_valid_byte_range(
			loc.byte_offset, loc.byte_length,
		);

		if has_bytes {
			extract_via_bytes(symbol, before, after)
		} else {
			extract_via_lines(symbol, before, after)
		}
	}
}

/// Extract code by seeking to the byte range.
/// Reads only the symbol bytes plus padding, then
/// uses the known line number to format output.
fn extract_via_bytes(
	symbol: &Symbol,
	ctx_before: usize,
	ctx_after: usize,
) -> RetrievalResult<String> {
	let loc = &symbol.location;
	// padding: ~120 chars/line * needed context
	let pad = (ctx_before + ctx_after + 50) * 120;

	let chunk = file_reader::read_byte_range(
		&loc.file, loc.byte_offset,
		loc.byte_length, pad,
	)?;

	let lines: Vec<&str> = chunk.lines().collect();

	// count newlines before byte_offset in the chunk
	let bytes_before =
		loc.byte_offset.saturating_sub(pad);
	let pre_offset = loc.byte_offset - bytes_before;
	let pre_lines =
		chunk[..pre_offset.min(chunk.len())]
			.matches('\n').count();

	// first file line = symbol line - pre_lines
	let first_line =
		loc.line.saturating_sub(pre_lines + 1);
	let sym_idx = loc.line.saturating_sub(first_line + 1);

	let sym_end =
		find_symbol_end(&lines, sym_idx, symbol);
	let start = sym_idx.saturating_sub(ctx_before);
	let end = (sym_end + ctx_after).min(lines.len());

	Ok(format_lines(
		&lines[start..end], first_line + start,
	))
}

/// Extract code via buffered line-by-line reading.
/// Reads only the needed line range from the file.
fn extract_via_lines(
	symbol: &Symbol,
	ctx_before: usize,
	ctx_after: usize,
) -> RetrievalResult<String> {
	let loc = &symbol.location;
	// 0-indexed line range to read
	let read_start =
		loc.line.saturating_sub(ctx_before + 1);
	let read_end = loc.line + 100 + ctx_after;

	let lines = file_reader::read_lines_range(
		&loc.file, read_start, read_end,
	)?;

	let refs: Vec<&str> =
		lines.iter().map(|line| line.as_str()).collect();

	let sym_idx =
		loc.line.saturating_sub(read_start + 1);
	let sym_end =
		find_symbol_end(&refs, sym_idx, symbol);
	let end = (sym_end + ctx_after).min(refs.len());

	Ok(format_lines(&refs[..end], read_start))
}

/// Format lines with 1-indexed line numbers.
fn format_lines(
	lines: &[&str],
	first_line_0idx: usize,
) -> String {
	let mut snippet = String::new();
	for (i, line) in lines.iter().enumerate() {
		let num = first_line_0idx + i + 1;
		snippet.push_str(
			&format!("{:4} | {}\n", num, line),
		);
	}
	snippet
}
