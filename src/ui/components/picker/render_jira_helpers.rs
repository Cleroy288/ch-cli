use ratatui::style::Color;

use crate::domain::jira::JiraSprint;
use crate::ui::styles::colors;

/// "done" = green, "indeterminate" = blue,
/// "new" (To Do) = gray.
pub fn status_to_color(
    category_key: &str,
) -> Color {
    match category_key {
        "done" => Color::Green,
        "indeterminate" => {
            colors::JIRA_IN_PROGRESS
        }
        _ => colors::SEGMENT_LABEL,
    }
}

/// Pads shorter strings to `max` width.
/// Uses char_indices for UTF-8 safe slicing.
pub fn truncate(
    text: &str,
    max: usize,
) -> String {
    if text.chars().count() <= max {
        format!("{:<width$}", text, width = max)
    } else {
        let end = text
            .char_indices()
            .nth(max - 3)
            .map(|(idx, _)| idx)
            .unwrap_or(text.len());
        format!("{}...", &text[..end])
    }
}

pub fn short_name(name: &str) -> String {
    if name.len() <= 12 {
        return name.to_string();
    }
    name.split_whitespace()
        .next()
        .unwrap_or(name)
        .to_string()
}

/// Index 1..=12 maps to 3-letter name.
const MONTHS: [&str; 13] = [
    "", "Jan", "Feb", "Mar", "Apr", "May",
    "Jun", "Jul", "Aug", "Sep", "Oct", "Nov",
    "Dec",
];

/// "2026-02-20T..." => "Feb 20"
pub fn short_date(iso: &str) -> String {
    if iso.len() < 10 {
        return iso.to_string();
    }
    let parts: Vec<&str> =
        iso[..10].split('-').collect();
    if parts.len() < 3 {
        return iso[..10].to_string();
    }
    let month_idx = parts[1]
        .parse::<usize>()
        .unwrap_or(0);
    let abbr = MONTHS
        .get(month_idx)
        .unwrap_or(&parts[1]);
    format!("{} {}", abbr, parts[2])
}

pub fn format_sprint_info(
    sprint: &JiraSprint,
) -> String {
    let start = short_date(&sprint.start_date);
    let end = short_date(&sprint.end_date);
    format!(
        "{} | {} \u{2192} {}",
        sprint.name, start, end,
    )
}
