use crate::domain::{FileReference, SymbolSelector};
use crate::message::MessageSegment;

use super::parser::RefSpan;

pub(crate) fn build_interleaved(
    input: &str,
    spans: &[RefSpan<'_>],
) -> Vec<MessageSegment> {
    let mut segments = Vec::new();
    let mut last_pos = 0;

    for span in spans {
        let (start, end, segment) = match span {
            RefSpan::File(fref) => (
                fref.start,
                fref.end,
                build_file_seg(fref),
            ),
            RefSpan::Symbol(sref) => (
                sref.start,
                sref.end,
                build_sym_seg(sref),
            ),
        };
        push_text(
            &mut segments, input, last_pos, start,
        );
        segments.push(segment);
        last_pos = end;
    }
    push_text(
        &mut segments, input, last_pos, input.len(),
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
        segments.push(
            MessageSegment::Text(text.to_string()),
        );
    }
}

fn build_file_seg(
    fref: &FileReference,
) -> MessageSegment {
    if fref.is_dir {
        MessageSegment::FolderReference {
            full_path: fref.full_path.to_string(),
            display_name: fref
                .display_name
                .to_string(),
        }
    } else {
        MessageSegment::FileReference {
            full_path: fref.full_path.to_string(),
            display_name: fref
                .display_name
                .to_string(),
        }
    }
}

fn build_sym_seg(
    sref: &SymbolSelector,
) -> MessageSegment {
    MessageSegment::SymbolReference {
        full_path: sref.file_path.to_string(),
        display_name: sref.display_text(),
        symbol_path: sref.symbol_path.clone(),
        source_code: None,
    }
}
