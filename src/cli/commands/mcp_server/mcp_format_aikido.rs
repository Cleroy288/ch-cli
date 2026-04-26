use std::collections::HashMap;
use std::hash::Hash;

use crate::domain::aikido::{Issue, SEV_ORDER};

use super::mcp_format_aikido_detail::or_dash;
use super::mcp_format_aikido_issue::issue_line;

pub use super::mcp_format_aikido_detail::{
	format_counts, format_detail,
};
pub use super::mcp_format_aikido_list::{
	format_containers, format_repos,
};

/// Format issues grouped by severity
pub fn format_grouped(
	issues: &[Issue],
) -> String {
	let groups =
		group_by(issues, |i| i.severity);
	let total = issues.len();
	let mut out =
		format!("Total: {total} issue(s)\n\n");
	for sev in SEV_ORDER {
		if let Some(bucket) = groups.get(&sev) {
			let label =
				sev.to_string().to_uppercase();
			let count = bucket.len();
			out.push_str(
				&format!("{label} ({count})\n"),
			);
			for issue in bucket {
				out.push_str(&format!(
					"  {}\n",
					issue_line(issue),
				));
			}
			out.push('\n');
		}
	}
	out
}

/// Format issues as flat sorted list
pub fn format_flat(
	issues: &[Issue],
) -> String {
	let mut sorted: Vec<&Issue> =
		issues.iter().collect();
	sorted.sort_by(|a, b| {
		b.severity_score.cmp(&a.severity_score)
	});
	sorted
		.iter()
		.map(|i| format_flat_line(i))
		.collect::<Vec<_>>()
		.join("\n")
}

/// Format issues as severity summary
pub fn format_summary(
	issues: &[Issue],
) -> String {
	let groups =
		group_by(issues, |i| i.severity);
	let mut out = format!(
		"Total: {} issue(s)\n",
		issues.len(),
	);
	for sev in SEV_ORDER {
		let count =
			groups.get(&sev).map_or(0, Vec::len);
		out.push_str(
			&format!("  {sev}: {count}\n"),
		);
	}
	out
}

fn group_by<T, K, F>(
	items: &[T],
	key_fn: F,
) -> HashMap<K, Vec<&T>>
where
	K: Eq + Hash,
	F: Fn(&T) -> K,
{
	let mut map: HashMap<K, Vec<&T>> =
		HashMap::new();
	for item in items {
		map.entry(key_fn(item))
			.or_default()
			.push(item);
	}
	map
}

fn format_flat_line(i: &Issue) -> String {
	format!(
		"[{}] #{} {} | {} | {} | {}",
		i.severity.to_string().to_uppercase(),
		i.id,
		i.issue_type,
		or_dash(&i.cve_id),
		or_dash(&i.affected_package),
		or_dash(&i.affected_file),
	)
}
