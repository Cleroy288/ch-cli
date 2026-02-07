//! Query decomposition into sub-queries

/// Common stop words to filter out
const STOP_WORDS: &[&str] = &[
	"how", "does", "do", "the", "a", "an", "is", "are",
	"was", "were", "what", "where", "when", "why", "which",
	"who", "work", "works", "working",
];

/// Decompose complex query into simpler sub-queries
#[doc(hidden)]
pub fn decompose_query(query: &str) -> Vec<String> {
	let mut sub_queries = Vec::new();
	let lower = query.to_lowercase();

	split_on_connectors(query, &lower, &mut sub_queries);
	extract_how_does_pattern(query, &lower, &mut sub_queries);
	extract_the_pattern(&lower, &mut sub_queries);

	sub_queries
}

/// Split query on connector words
fn split_on_connectors(
	query: &str,
	lower: &str,
	sub_queries: &mut Vec<String>,
) {
	let connectors =
		["and", "with", "using", "via", "through"];

	for connector in connectors {
		let pattern = format!(" {} ", connector);
		if lower.contains(&pattern) {
			let parts: Vec<&str> =
				query.split(&pattern).collect();
			if parts.len() == 2 {
				sub_queries.push(parts[0].trim().to_string());
				sub_queries.push(parts[1].trim().to_string());
			}
		}
	}
}

/// Extract key concept from "how does X work" pattern
fn extract_how_does_pattern(
	query: &str,
	lower: &str,
	sub_queries: &mut Vec<String>,
) {
	let starts_how = lower.starts_with("how does ");
	let ends_work = lower.ends_with(" work");
	if starts_how && ends_work {
		let mid = &query[9..query.len() - 5].trim();
		if !mid.is_empty() {
			sub_queries.push(mid.to_string());
			sub_queries
				.push(format!("{} implementation", mid));
		}
	}
}

/// Extract key words from "the X Y" pattern
fn extract_the_pattern(
	lower: &str,
	sub_queries: &mut Vec<String>,
) {
	if lower.contains("the ") {
		if let Some(rest) = lower.split("the ").nth(1) {
			let key_words: Vec<&str> = rest
				.split_whitespace()
				.filter(|w| !STOP_WORDS.contains(w))
				.take(3)
				.collect();
			if !key_words.is_empty() {
				sub_queries.push(key_words.join(" "));
			}
		}
	}
}
