use crate::service::search::types::SymbolDetails;

pub fn format_symbol_details(
	details: &SymbolDetails,
) -> String {
	let mut out = format_header(details);
	append_callers(&mut out, details);
	append_source(&mut out, details);
	out
}

fn format_header(
	details: &SymbolDetails,
) -> String {
	let sym = &details.symbol;
	let mut out = format!(
		"[{}] {}\n  {}:{}\n",
		sym.kind,
		sym.name,
		sym.location.file.display(),
		sym.location.line,
	);
	if let Some(sig) = &sym.signature {
		out.push_str(&format!("  sig: {sig}\n"));
	}
	if let Some(doc) = &sym.doc_comment {
		out.push_str(&format!("  doc: {doc}\n"));
	}
	out
}

/// Append caller lines to output
fn append_callers(
	out: &mut String,
	details: &SymbolDetails,
) {
	if details.callers.is_empty() {
		return;
	}
	out.push_str("  callers:\n");
	for cal in &details.callers {
		let name = cal.caller_name
			.as_deref()
			.unwrap_or("?");
		out.push_str(&format!(
			"    {name} @ {}:{}\n",
			cal.file.display(),
			cal.line,
		));
	}
}

/// Append source snippet and reference count
fn append_source(
	out: &mut String,
	details: &SymbolDetails,
) {
	if let Some(src) = &details.source_code {
		out.push_str("  source:\n");
		for line in src.lines().take(20) {
			out.push_str(&format!("    {line}\n"));
		}
	}
	if !details.references.is_empty() {
		let count = details.references.len();
		out.push_str(&format!(
			"  references: {count} locations\n",
		));
	}
}
