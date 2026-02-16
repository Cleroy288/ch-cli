//! Query decomposition into sub-queries

/// Common stop words to filter out
const STOP_WORDS: &[&str] = &[
	"how", "does", "do", "the", "a", "an", "is", "are",
	"was", "were", "what", "where", "when", "why", "which",
	"who", "work", "works", "working",
];

/// Minimum identifier length to attempt expansion
const MIN_IDENT_LEN: usize = 3;

/// Minimum uppercase chars for CamelCase detection
const MIN_CAMEL_UPPERS: usize = 2;

/// Decompose complex query into simpler sub-queries
#[doc(hidden)]
pub fn decompose_query(query: &str) -> Vec<String> {
	let mut sub_queries = Vec::new();
	let lower = query.to_lowercase();

	split_on_connectors(query, &lower, &mut sub_queries);
	extract_how_does_pattern(
		query, &lower, &mut sub_queries,
	);
	extract_the_pattern(&lower, &mut sub_queries);
	expand_identifiers(query, &mut sub_queries);
	sub_queries
}

/// Extract "the X Y" pattern
fn extract_the_pattern(
	lower: &str,
	sub_queries: &mut Vec<String>,
) {
	if !lower.contains("the ") {
		return;
	}
	if let Some(rest) = lower.split("the ").nth(1) {
		let words: Vec<&str> = rest
			.split_whitespace()
			.filter(|word| !STOP_WORDS.contains(word))
			.take(3)
			.collect();
		if !words.is_empty() {
			sub_queries.push(words.join(" "));
		}
	}
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
				sub_queries
					.push(parts[0].trim().to_string());
				sub_queries
					.push(parts[1].trim().to_string());
			}
		}
	}
}

/// Extract key concept from "how does X work"
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

/// Expand CamelCase and snake_case identifiers
fn expand_identifiers(
	query: &str,
	sub_queries: &mut Vec<String>,
) {
	let mut changed = false;
	let expanded: Vec<String> = query
		.split_whitespace()
		.map(|word| {
			expand_single_word(word, &mut changed)
		})
		.collect();

	if changed {
		let result = expanded.join(" ").to_lowercase();
		if result != query.to_lowercase() {
			sub_queries.push(result);
		}
	}
}

/// Expand a CamelCase or snake_case word to words
fn expand_single_word(
	word: &str,
	changed: &mut bool,
) -> String {
	if word.contains('_') && word.len() >= MIN_IDENT_LEN
	{
		*changed = true;
		return word.replace('_', " ");
	}
	expand_camel_case(word, changed)
}

/// Expand CamelCase word by inserting spaces
fn expand_camel_case(
	word: &str,
	changed: &mut bool,
) -> String {
	let uppers = word
		.chars()
		.filter(|chr| chr.is_uppercase())
		.count();
	if uppers < MIN_CAMEL_UPPERS
		|| word.len() < MIN_IDENT_LEN
	{
		return word.to_string();
	}
	*changed = true;
	let mut out =
		String::with_capacity(word.len() + 4);
	for (idx, chr) in word.chars().enumerate() {
		if idx > 0 && chr.is_uppercase() {
			out.push(' ');
		}
		out.push(chr);
	}
	out
}
