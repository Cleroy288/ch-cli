/// Max value for unparseable sprint components
const MAX_U16: u16 = u16::MAX;

/// Max value for unparseable quarter
const MAX_U8: u8 = u8::MAX;

///
/// Names that don't match the expected format
/// sort after all valid sprint names.
/// Format: "{Name} {N} - Q{Q} {Year}"
pub fn sprint_sort_key(
    name: &str,
) -> (u16, u8, u16) {
    let Some((left, right)) =
        name.split_once(" - ")
    else {
        return (MAX_U16, MAX_U8, MAX_U16);
    };
    let number = parse_event_number(left);
    let (year, quarter) = parse_period(right);
    (year, quarter, number)
}

/// Extract the number from "Sprint 2" or "Bridge 4"
fn parse_event_number(left: &str) -> u16 {
    left.split_whitespace()
        .last()
        .and_then(|num| num.parse().ok())
        .unwrap_or(MAX_U16)
}

/// Extract (year, quarter) from "Q1 2026"
fn parse_period(right: &str) -> (u16, u8) {
    let parts: Vec<&str> =
        right.split_whitespace().collect();
    if parts.len() < 2 {
        return (MAX_U16, MAX_U8);
    }
    let quarter = parts[0]
        .strip_prefix('Q')
        .and_then(|q_num| q_num.parse().ok())
        .unwrap_or(MAX_U8);
    let year = parts[1]
        .parse()
        .unwrap_or(MAX_U16);
    (year, quarter)
}
