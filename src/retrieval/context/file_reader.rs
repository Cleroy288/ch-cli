//! Buffered File Reading Utilities
//!
//! Provides memory-efficient file reading for context
//! expansion. Uses BufReader to avoid loading entire
//! files into memory. Uses Seek + byte ranges when
//! symbol byte offsets are available.

use std::fs::File;
use std::io::{self, BufRead, BufReader, Read, Seek};
use std::path::Path;

/// Read only the lines in [start, end) from a file.
/// Uses BufReader to avoid loading the entire file.
/// Line indices are 0-based. Returns owned strings.
pub fn read_lines_range(
	path: &Path,
	start: usize,
	end: usize,
) -> io::Result<Vec<String>> {
	let file = File::open(path)?;
	let reader = BufReader::new(file);
	let capacity = end.saturating_sub(start);
	let mut result = Vec::with_capacity(capacity);

	for (idx, line) in reader.lines().enumerate() {
		if idx >= end {
			break;
		}
		if idx >= start {
			result.push(line?);
		}
	}

	Ok(result)
}

/// Read a byte range from a file using Seek.
/// Reads from `offset` for `length` bytes, with extra
/// padding for context lines. Returns the raw string.
pub fn read_byte_range(
	path: &Path,
	offset: usize,
	length: usize,
	padding: usize,
) -> io::Result<String> {
	let file = File::open(path)?;
	let file_len = file.metadata()?.len() as usize;

	// compute seek bounds with padding
	let seek_start = offset.saturating_sub(padding);
	let read_end =
		(offset + length + padding).min(file_len);
	let read_len = read_end - seek_start;

	read_chunk(file, seek_start as u64, read_len)
}

/// Read a chunk of bytes from a file at an offset.
/// Returns the chunk as a String (lossy UTF-8).
fn read_chunk(
	mut file: File,
	seek_pos: u64,
	read_len: usize,
) -> io::Result<String> {
	file.seek(io::SeekFrom::Start(seek_pos))?;
	let mut buf = vec![0u8; read_len];
	let n = file.read(&mut buf)?;
	buf.truncate(n);
	Ok(String::from_utf8_lossy(&buf).into_owned())
}

/// Check if a byte offset is valid for range reading.
/// Returns false if offset is zero or length is zero.
pub fn has_valid_byte_range(
	byte_offset: usize,
	byte_length: usize,
) -> bool {
	byte_offset > 0 && byte_length > 0
}
