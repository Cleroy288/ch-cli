use ratatui::{
    style::Style,
    text::{Line, Span},
};

use crate::app::App;
use crate::app::handlers::input_paste::PasteBlock;
use crate::domain::{FileReference, SymbolSelector};
use crate::ui::styles::{self, colors};

/// A positioned span for rendering
struct StyledSpan {
    start: usize,
    end: usize,
    style: Style,
}

/// Build styled lines — one per logical line.
pub(crate) fn build_styled_input_lines(
    app: &App,
) -> Vec<Line<'static>> {
    let input = app.input();
    let logical: Vec<&str> =
        input.split('\n').collect();
    if app.file_references().is_empty()
        && app.symbol_selectors().is_empty()
        && app.paste_blocks.is_empty()
    {
        return plain_lines(&logical);
    }
    let spans = collect_styled_spans(
        app.file_references(),
        app.symbol_selectors(),
        &app.paste_blocks,
    );
    styled_lines(input, &logical, &spans)
}

fn plain_lines(
    logical: &[&str],
) -> Vec<Line<'static>> {
    let style =
        Style::default().fg(colors::INPUT_TEXT);
    logical
        .iter()
        .map(|l| {
            Line::from(Span::styled(
                (*l).to_owned(),
                style,
            ))
        })
        .collect()
}

fn styled_lines(
    input: &str,
    logical: &[&str],
    spans: &[StyledSpan],
) -> Vec<Line<'static>> {
    let mut result = Vec::with_capacity(
        logical.len(),
    );
    let mut offset = 0;
    for line_text in logical {
        let end = offset + line_text.len();
        let line = build_line_from_ranges(
            input, offset, end, spans,
        );
        result.push(line);
        offset = end + 1; // skip '\n'
    }
    result
}

fn build_line_from_ranges(
    input: &str,
    start: usize,
    end: usize,
    ranges: &[StyledSpan],
) -> Line<'static> {
    let normal =
        Style::default().fg(colors::INPUT_TEXT);
    let mut result: Vec<Span<'static>> = Vec::new();
    let mut pos = start;
    for range in ranges {
        if range.end <= start || range.start >= end {
            continue;
        }
        let rs = range.start.max(start);
        let re = range.end.min(end);
        if pos < rs {
            result.push(Span::styled(
                input[pos..rs].to_owned(),
                normal,
            ));
        }
        result.push(Span::styled(
            input[rs..re].to_owned(),
            range.style,
        ));
        pos = re;
    }
    if pos < end {
        result.push(Span::styled(
            input[pos..end].to_owned(),
            normal,
        ));
    }
    if result.is_empty() {
        result.push(Span::styled(
            String::new(), normal,
        ));
    }
    Line::from(result)
}

/// Symbol selectors subsume their file reference
/// (same start), so skip file refs covered by one.
fn collect_styled_spans(
    file_refs: &[FileReference],
    sym_refs: &[SymbolSelector],
    pastes: &[PasteBlock],
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
    let paste_style = Style::default()
        .fg(colors::TEXT_LIGHT)
        .italic();
    for paste in pastes {
        spans.push(StyledSpan {
            start: paste.start,
            end: paste.end,
            style: paste_style,
        });
    }
    spans.sort_by_key(|span| span.start);
    spans
}

fn is_subsumed_by_symbol(
    fref: &FileReference,
    sym_refs: &[SymbolSelector],
) -> bool {
    sym_refs.iter().any(|sref| {
        sref.start <= fref.start
            && sref.end >= fref.end
    })
}
