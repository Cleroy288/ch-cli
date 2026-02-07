//! XML Formatting for Usage Sections
//!
//! Formats UsageInfo collections into XML, grouped by file
//! path for organized output.

use super::context_xml::escape_xml;
use super::graph_walker_usage::UsageInfo;

/// Format usages section as XML, grouped by file
pub fn format_usages_xml(usages: &[UsageInfo]) -> String {
	if usages.is_empty() {
		return String::new();
	}

	let mut xml = format!(
		"  <usages total=\"{}\">\n",
		usages.len(),
	);

	// group usages by file path
	let mut by_file: std::collections::HashMap<
		&std::path::Path,
		Vec<&UsageInfo>,
	> = std::collections::HashMap::new();

	for usage in usages {
		by_file
			.entry(usage.file.as_path())
			.or_default()
			.push(usage);
	}

	// sort file paths for consistent output
	let mut files: Vec<_> = by_file.keys().collect();
	files.sort();

	for file in files {
		let file_usages = &by_file[file];
		xml.push_str(&format!(
			"    <file path=\"{}\">\n",
			file.display(),
		));

		for usage in file_usages {
			xml.push_str(&format_single_usage(usage));
		}

		xml.push_str("    </file>\n");
	}

	xml.push_str("  </usages>\n");
	xml
}

/// Format a single usage entry as XML
fn format_single_usage(usage: &UsageInfo) -> String {
	let mut xml = format!(
		"      <usage line=\"{}\" context=\"{:?}\"",
		usage.line, usage.context,
	);

	if let Some(ref containing) = usage.containing_symbol {
		xml.push_str(&format!(" in=\"{}\"", containing));
	}
	xml.push_str(">\n");

	if let Some(ref snippet) = usage.snippet {
		let escaped = escape_xml(snippet);
		xml.push_str(&format!("        {}\n", escaped));
	}

	xml.push_str("      </usage>\n");
	xml
}
