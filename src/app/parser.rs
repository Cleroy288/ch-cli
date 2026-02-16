use crate::domain::{FileReference, SymbolSelector};
use crate::message::{MessageSegment, UserMessage};

/// Parse input string with file and symbol references
/// into a structured UserMessage.
///
/// Pure function with no I/O or side effects.
pub fn parse_input_to_message(
    input: String,
    file_refs: &[FileReference],
    symbol_refs: &[SymbolSelector],
) -> Option<UserMessage> {
    let trimmed = input.trim_end().to_string();
    if trimmed.trim().is_empty() {
        return None;
    }

    let segments = build_message_segments(
        &trimmed,
        file_refs,
        symbol_refs,
    );
    Some(UserMessage::new(segments, trimmed))
}

/// Build message segments from all reference types.
///
/// Merges file refs and symbol refs sorted by position,
/// then interleaves with text segments.
fn build_message_segments(
    input: &str,
    file_refs: &[FileReference],
    symbol_refs: &[SymbolSelector],
) -> Vec<MessageSegment> {
    let spans = collect_sorted_spans(
        file_refs,
        symbol_refs,
    );
    if spans.is_empty() {
        return build_text_only(input);
    }
    build_interleaved(input, &spans)
}

/// A span that can be either file or symbol ref
enum RefSpan<'a> {
    File(&'a FileReference),
    Symbol(&'a SymbolSelector),
}

/// Collect and sort all reference spans by position
fn collect_sorted_spans<'a>(
    file_refs: &'a [FileReference],
    symbol_refs: &'a [SymbolSelector],
) -> Vec<RefSpan<'a>> {
    let mut spans: Vec<RefSpan<'a>> = Vec::new();
    for fref in file_refs {
        spans.push(RefSpan::File(fref));
    }
    for sref in symbol_refs {
        spans.push(RefSpan::Symbol(sref));
    }
    spans.sort_by_key(|span| match span {
        RefSpan::File(fref) => fref.start,
        RefSpan::Symbol(sref) => sref.start,
    });
    spans
}

/// Build a single text segment when no refs exist
fn build_text_only(
    input: &str,
) -> Vec<MessageSegment> {
    if input.is_empty() {
        return Vec::new();
    }
    vec![MessageSegment::Text(input.to_string())]
}

/// Build segments interleaved with references
fn build_interleaved(
    input: &str,
    spans: &[RefSpan<'_>],
) -> Vec<MessageSegment> {
    let mut segments = Vec::new();
    let mut last_pos = 0;

    for span in spans {
        let (start, end, segment) = match span {
            RefSpan::File(fref) => {
                (fref.start, fref.end, build_file_seg(fref))
            }
            RefSpan::Symbol(sref) => {
                (sref.start, sref.end, build_sym_seg(sref))
            }
        };
        push_text(&mut segments, input, last_pos, start);
        segments.push(segment);
        last_pos = end;
    }
    push_text(
        &mut segments,
        input,
        last_pos,
        input.len(),
    );
    segments
}

/// Push a text segment if range has content
fn push_text(
    segments: &mut Vec<MessageSegment>,
    input: &str,
    from: usize,
    end: usize,
) {
    if from >= end {
        return;
    }
    let text = &input[from..end];
    if !text.trim().is_empty() {
        segments
            .push(MessageSegment::Text(text.to_string()));
    }
}

/// Build a file or folder reference segment
fn build_file_seg(
    fref: &FileReference,
) -> MessageSegment {
    if fref.is_dir {
        MessageSegment::FolderReference {
            full_path: fref.full_path.as_string(),
            display_name: fref.display_name.as_string(),
        }
    } else {
        MessageSegment::FileReference {
            full_path: fref.full_path.as_string(),
            display_name: fref.display_name.as_string(),
        }
    }
}

/// Build a symbol reference segment
fn build_sym_seg(
    sref: &SymbolSelector,
) -> MessageSegment {
    MessageSegment::SymbolReference {
        full_path: sref.file_path.as_string(),
        display_name: sref.display_text(),
        symbol_path: sref.symbol_path.clone(),
        source_code: None,
    }
}
