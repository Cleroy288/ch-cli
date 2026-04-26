use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::message::MessageSegment;
use crate::ui::styles::colors;

use super::ref_spans::{
    RefSpanInfo, append_ref_spans, append_text_spans,
};

pub(super) fn build_segment_line(
    segment: &MessageSegment,
    idx: usize,
) -> Line<'static> {
    let mut spans = build_segment_prefix(idx);
    append_segment_content(&mut spans, segment);
    Line::from(spans)
}

fn build_segment_prefix(
    idx: usize,
) -> Vec<Span<'static>> {
    vec![
        Span::raw("  "),
        Span::styled(
            format!("[{}] ", idx),
            Style::default().fg(colors::PLACEHOLDER),
        ),
    ]
}

/// Append segment-specific content spans
fn append_segment_content(
    spans: &mut Vec<Span<'static>>,
    segment: &MessageSegment,
) {
    match segment {
        MessageSegment::Text(text) => {
            append_text_spans(spans, text);
        }
        MessageSegment::FileReference {
            full_path, display_name,
        }
        | MessageSegment::SymbolReference {
            display_name, full_path, ..
        } => {
            append_file_ref_spans(
                spans, display_name, full_path,
            );
        }
        MessageSegment::FolderReference {
            full_path, display_name,
        } => {
            append_folder_ref_spans(
                spans, display_name, full_path,
            );
        }
    }
}

/// Append file reference spans
fn append_file_ref_spans(
    spans: &mut Vec<Span<'static>>,
    display_name: &str,
    full_path: &str,
) {
    append_ref_spans(
        spans,
        &RefSpanInfo {
            label: "File: ",
            label_color: colors::FILE_REF_BG,
            display_name,
            full_path,
        },
    );
}

/// Append folder reference spans
fn append_folder_ref_spans(
    spans: &mut Vec<Span<'static>>,
    display_name: &str,
    full_path: &str,
) {
    append_ref_spans(
        spans,
        &RefSpanInfo {
            label: "Folder: ",
            label_color: colors::FOLDER_REF_BG,
            display_name,
            full_path,
        },
    );
}
