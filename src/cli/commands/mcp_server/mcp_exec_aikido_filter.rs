use serde_json::Value;

use crate::domain::aikido::{
	Issue, IssueCountFilters, IssueFilters,
};
use crate::service::aikido::AikidoClient;

use super::mcp_format_aikido;
use super::mcp_helpers::{
	extract_string, wrap_text_content,
};
use super::mcp_types::error_response;

/// Fetch issues with severity/repo filters
pub fn exec_get_issues(
	id: Value,
	args: &Value,
	client: &AikidoClient,
) -> String {
	let filters = build_issue_filters(args);
	let fmt = extract_string(args, "format")
		.unwrap_or_else(|| "grouped".into());
	match client.fetch_issues(&filters) {
		Ok(issues) => {
			let text =
				format_by_mode(&fmt, &issues);
			wrap_text_content(id, &text)
		}
		Err(e) => error_response(
			id, -32000, &e.to_string(),
		),
	}
}

/// Fetch issue counts with optional filters
pub fn exec_issue_counts(
	id: Value,
	args: &Value,
	client: &AikidoClient,
) -> String {
	let filters = build_count_filters(args);
	match client.fetch_counts(&filters) {
		Ok(counts) => {
			let text = mcp_format_aikido
				::format_counts(&counts);
			wrap_text_content(id, &text)
		}
		Err(e) => error_response(
			id, -32000, &e.to_string(),
		),
	}
}

fn build_issue_filters(
	args: &Value,
) -> IssueFilters {
	let severities = extract_string(
		args, "severities",
	)
	.map(|s| {
		s.split(',')
			.map(|v| v.trim().to_string())
			.collect()
	});
	IssueFilters {
		repo_name: extract_string(
			args, "repo_name",
		),
		container_name: extract_string(
			args, "container_name",
		),
		issue_type: extract_string(
			args, "issue_type",
		),
		severities,
	}
}

fn build_count_filters(
	args: &Value,
) -> IssueCountFilters {
	IssueCountFilters {
		repo_name: extract_string(
			args, "repo_name",
		),
		container_repo_id: args
			.get("container_repo_id")
			.and_then(Value::as_u64),
	}
}

fn format_by_mode(
	mode: &str,
	issues: &[Issue],
) -> String {
	match mode {
		"flat" => {
			mcp_format_aikido::format_flat(issues)
		}
		"summary" => {
			mcp_format_aikido::format_summary(
				issues,
			)
		}
		_ => {
			mcp_format_aikido::format_grouped(
				issues,
			)
		}
	}
}
