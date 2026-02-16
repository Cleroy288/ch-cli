//! Input text styling for file/folder/symbol references.
//!
//! Builds styled `Line` widgets by highlighting
//! file references, folder references, and symbol
//! selectors within the user's input text.

use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::app::App;
use crate::domain::{FileReference, SymbolSelector};
use crate::ui::styles::{self, colors};

/// A positioned span for rendering
struct StyledSpan {
    start: usize,
    end: usize,
    style: Style,
}

/// Build a styled line with references highlighted.
pub(crate) fn build_styled_input_line(
    app: &App,
) -> Line<'static> {
    let input = app.input();
    let file_refs = app.file_references();
    let sym_refs = app.symbol_selectors();

    let has_refs =
        !file_refs.is_empty() || !sym_refs.is_empty();
    if !has_refs {
        return Line::from(Span::styled(
            input.to_string(),
            Style::default().fg(colors::INPUT_TEXT),
        ));
    }

    let spans =
        collect_styled_spans(file_refs, sym_refs);
    build_spans_from_ranges(input, &spans)
}

/// Collect all styled spans sorted by position.
///
/// Symbol selectors subsume their file reference
/// (same start), so skip file refs covered by one.
fn collect_styled_spans(
    file_refs: &[FileReference],
    sym_refs: &[SymbolSelector],
) -> Vec<StyledSpan> {
    let mut spans = Vec::new();
    for fref in file_refs {
        if is_subsumed_by_symbol(fref, sym_refs) {
            continue;
        }
        let style = if fref.is_dir {
            styles::folder_reference_style()
        } else {
            styles::file_reference_style()
        };
        spans.push(StyledSpan {
            start: fref.start,
            end: fref.end,
            style,
        });
    }
    for sref in sym_refs {
        spans.push(StyledSpan {
            start: sref.start,
            end: sref.end,
            style: styles::file_reference_style(),
        });
    }
    spans.sort_by_key(|span| span.start);
    spans
}

/// Check if a file ref is fully inside a symbol ref
fn is_subsumed_by_symbol(
    fref: &FileReference,
    sym_refs: &[SymbolSelector],
) -> bool {
    sym_refs.iter().any(|sref| {
        sref.start <= fref.start
            && sref.end >= fref.end
    })
}

/// Build Line from sorted styled span ranges
fn build_spans_from_ranges(
    input: &str,
    ranges: &[StyledSpan],
) -> Line<'static> {
    let mut result: Vec<Span<'static>> = Vec::new();
    let mut last_pos = 0;

    for range in ranges {
        let (start, end) =
            (range.start, range.end);
        push_normal(
            &mut result, input, last_pos, start,
        );
        push_styled_span(
            &mut result, input, range,
        );
        last_pos = end;
    }
    push_normal(
        &mut result, input, last_pos, input.len(),
    );
    Line::from(result)
}

/// Push a normal (unstyled) text span
fn push_normal(
    spans: &mut Vec<Span<'static>>,
    input: &str,
    from: usize,
    end: usize,
) {
    if from >= end || from >= input.len() {
        return;
    }
    let actual_end = end.min(input.len());
    let text = &input[from..actual_end];
    spans.push(Span::styled(
        text.to_string(),
        Style::default().fg(colors::INPUT_TEXT),
    ));
}

/// Push a styled reference span from a StyledSpan
fn push_styled_span(
    spans: &mut Vec<Span<'static>>,
    input: &str,
    range: &StyledSpan,
) {
    let (start, end) = (range.start, range.end);
    if start >= input.len() || end > input.len() {
        return;
    }
    let text = &input[start..end];
    spans.push(Span::styled(
        text.to_string(),
        range.style,
    ));
}
