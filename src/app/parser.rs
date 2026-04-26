use crate::domain::{FileReference, SymbolSelector};
use crate::message::{MessageSegment, UserMessage};

/// into a structured UserMessage.
///
/// Pure function with no I/O or side effects.
pub fn parse_input_to_message(
    input: String,
    file_refs: &[FileReference],
    symbol_refs: &[SymbolSelector],
) -> Option<UserMessage> {
    let trimmed = input.trim_end().to_string();
    if trimmed.trim_start().is_empty() {
        return None;
    }

    let segments = build_message_segments(
        &trimmed,
        file_refs,
        symbol_refs,
    );
    Some(UserMessage::new(segments, trimmed))
}

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
    super::parser_interleave::build_interleaved(
        input, &spans,
    )
}

/// A span that can be either file or symbol ref
pub(crate) enum RefSpan<'a> {
    File(&'a FileReference),
    Symbol(&'a SymbolSelector),
}

fn collect_sorted_spans<'a>(
    file_refs: &'a [FileReference],
    symbol_refs: &'a [SymbolSelector],
) -> Vec<RefSpan<'a>> {
    let mut spans: Vec<RefSpan<'a>> = file_refs
        .iter()
        .map(RefSpan::File)
        .chain(symbol_refs.iter().map(RefSpan::Symbol))
        .collect();
    spans.sort_by_key(|span| match span {
        RefSpan::File(fref) => fref.start,
        RefSpan::Symbol(sref) => sref.start,
    });
    spans
}

fn build_text_only(
    input: &str,
) -> Vec<MessageSegment> {
    if input.is_empty() {
        return Vec::new();
    }
    vec![MessageSegment::Text(input.to_string())]
}
