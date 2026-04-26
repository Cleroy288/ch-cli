use crate::service::search::types::{
	CallerHit, SearchResultHit,
};
use crate::service::search::types_navigation::{
	DefinitionHit, SymbolEntry,
};

/// No results placeholder
const NO_RESULTS: &str = "(no results)";

pub fn format_search_hits(
	hits: &[SearchResultHit],
) -> String {
	if hits.is_empty() {
		return NO_RESULTS.to_string();
	}
	hits.iter()
		.map(format_one_hit)
		.collect::<Vec<_>>()
		.join("\n---\n")
}

pub fn format_definitions(
	hits: &[DefinitionHit],
) -> String {
	if hits.is_empty() {
		return NO_RESULTS.to_string();
	}
	hits.iter()
		.map(|def| {
			format!(
				"{} [{}] {}:{}",
				def.fqn,
				def.symbol.kind,
				def.symbol.location.file.display(),
				def.symbol.location.line,
			)
		})
		.collect::<Vec<_>>()
		.join("\n")
}

pub fn format_callers(
	callers: &[CallerHit],
) -> String {
	if callers.is_empty() {
		return NO_RESULTS.to_string();
	}
	callers
		.iter()
		.map(|cal| {
			let name = cal.caller_name
				.as_deref()
				.unwrap_or("?");
			format!(
				"{} @ {}:{} — {}",
				name,
				cal.file.display(),
				cal.line,
				cal.context.trim(),
			)
		})
		.collect::<Vec<_>>()
		.join("\n")
}

pub fn format_symbol_list(
	entries: &[SymbolEntry],
) -> String {
	if entries.is_empty() {
		return NO_RESULTS.to_string();
	}
	entries
		.iter()
		.map(|ent| {
			format!(
				"[{}] {} — {}:{}",
				ent.symbol.kind,
				ent.fqn,
				ent.symbol.location.file.display(),
				ent.symbol.location.line,
			)
		})
		.collect::<Vec<_>>()
		.join("\n")
}

fn format_one_hit(hit: &SearchResultHit) -> String {
	let sym = &hit.symbol;
	let loc = &sym.location;
	format!(
		"[{:.3}] {} [{}] {}:{}",
		hit.score,
		sym.name,
		sym.kind,
		loc.file.display(),
		loc.line,
	)
}
