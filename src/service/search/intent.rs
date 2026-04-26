use super::boost::QueryIntent;

const STOP_WORDS: &[&str] = &[
	"how", "where", "what", "why", "when", "which",
	"who", "whom", "the", "a", "an", "and", "or",
	"but", "if", "then", "else", "is", "are", "was",
	"were", "be", "been", "being", "have", "has",
	"had", "do", "does", "did", "done", "can",
	"could", "would", "should", "will", "shall",
	"may", "might", "must", "to", "for", "in", "of",
	"on", "at", "by", "with", "from", "about",
	"find", "get", "show", "list", "explain",
	"describe", "tell", "give", "work", "works",
	"use", "uses", "used", "call", "calls", "called",
	"define", "defined", "definition", "this", "that",
	"it", "i", "me", "my", "we", "our", "all", "any",
	"some", "no", "not", "only", "just", "code",
	"file", "function", "class", "method", "module",
];

pub struct ParsedIntent {
	pub intent: QueryIntent,
	pub symbols: Vec<String>,
}

pub fn parse_query_intent(
	query: &str,
) -> ParsedIntent {
	let symbols = extract_identifiers(query);
	let intent = detect_intent(query);
	ParsedIntent { intent, symbols }
}

pub fn extract_identifiers(
	query: &str,
) -> Vec<String> {
	query
		.split(|chr: char| {
			!chr.is_alphanumeric() && chr != '_'
		})
		.filter(|w| w.len() >= 2)
		.filter(|w| is_code_identifier(w))
		.map(String::from)
		.collect()
}

fn is_code_identifier(word: &str) -> bool {
	let lower = word.to_lowercase();
	if STOP_WORDS.contains(&lower.as_str()) {
		return false;
	}
	let is_camel = word
		.chars()
		.any(|chr| chr.is_uppercase())
		&& word.chars().any(|chr| chr.is_lowercase());
	let is_snake = word.contains('_');
	is_camel || is_snake
}

type IntentRule = (&'static [&'static str], QueryIntent);

const INTENT_RULES: &[IntentRule] = &[
	(&["where", "definition", "defined"],
		QueryIntent::FindDefinition),
	(&["used", "references", "usages"],
		QueryIntent::FindUsages),
	(&["how", "explain", "understand"],
		QueryIntent::Understand),
	(&["fix", "change", "modify"],
		QueryIntent::Modify),
	(&["bug", "error", "debug"],
		QueryIntent::Debug),
];

fn detect_intent(query: &str) -> QueryIntent {
	let lower = query.to_lowercase();
	for (words, intent) in INTENT_RULES {
		if words.iter().any(|w| has_word(&lower, w)) {
			return *intent;
		}
	}
	QueryIntent::Search
}

fn has_word(query: &str, word: &str) -> bool {
	query.match_indices(word).any(|(idx, _)| {
		let before_ok = idx == 0
			|| query[..idx]
				.chars()
				.next_back()
				.is_none_or(|c| {
					!c.is_alphanumeric()
				});
		let end = idx + word.len();
		let after_ok = end >= query.len()
			|| query[end..]
				.chars()
				.next()
				.is_none_or(|c| {
					!c.is_alphanumeric()
				});
		before_ok && after_ok
	})
}
