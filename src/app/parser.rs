use crate::domain::FileReference;
use crate::message::{MessageSegment, UserMessage};

/// Parse input string and file references into a structured UserMessage.
///
/// This is a pure function with no I/O or side effects, following the
/// "Functional Core, Imperative Shell" pattern. It can be unit tested in isolation.
///
/// # Arguments
/// * `input` - The raw input string to parse
/// * `file_refs` - File references found in the input
///
/// # Returns
/// A `UserMessage` with parsed segments,
/// or None if input is empty/whitespace-only
pub fn parse_input_to_message(
    input: String,
    file_refs: &[FileReference],
) -> Option<UserMessage> {
    // Trim trailing whitespace
    let trimmed_input = input.trim_end().to_string();

    // Don't create message for empty/whitespace-only input
    if trimmed_input.trim().is_empty() {
        return None;
    }

    let segments = build_message_segments(&trimmed_input, file_refs);

    // Create the message with raw input
    Some(UserMessage::new(segments, trimmed_input))
}

/// Build message segments from input and file references.
///
/// Segments are created by:
/// 1. Sorting file references by position
/// 2. Extracting text between references
/// 3. Creating segment types based on reference type (file/folder)
fn build_message_segments(
    input: &str,
    file_refs: &[FileReference],
) -> Vec<MessageSegment> {
    let mut segments = Vec::new();

    // Handle case with no file references
    if file_refs.is_empty() {
        if !input.is_empty() {
            segments.push(MessageSegment::Text(input.to_string()));
        }
        return segments;
    }

    // Sort file references by start position
    let mut sorted_refs: Vec<&FileReference> = file_refs.iter().collect();
    sorted_refs.sort_by_key(|r| r.start);

    let mut last_pos = 0;

    for file_ref in sorted_refs {
        // Add text segment before this reference
        if last_pos < file_ref.start {
            let text = &input[last_pos..file_ref.start];
            // Only add non-whitespace text segments
            if !text.trim().is_empty() {
                segments.push(MessageSegment::Text(text.to_string()));
            }
        }

        // Add file or folder reference segment
        if file_ref.is_dir {
            segments.push(MessageSegment::FolderReference {
                full_path: file_ref.full_path.as_string(),
                display_name: file_ref.display_name.as_string(),
            });
        } else {
            segments.push(MessageSegment::FileReference {
                full_path: file_ref.full_path.as_string(),
                display_name: file_ref.display_name.as_string(),
            });
        }

        last_pos = file_ref.end;
    }

    // Add remaining text after last reference
    if last_pos < input.len() {
        let text = &input[last_pos..];
        // Only add non-whitespace text segments
        if !text.trim().is_empty() {
            segments.push(MessageSegment::Text(text.to_string()));
        }
    }

    segments
}

