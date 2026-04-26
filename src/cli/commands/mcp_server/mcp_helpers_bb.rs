use serde_json::Value;

use crate::service::atlassian::client_bb::BbClient;
use crate::service::atlassian
	::client_bb_paginate::get_all_pages;

use super::mcp_helpers::wrap_text_content;
use super::mcp_types::error_response;

/// Route entry for a Bitbucket API endpoint
pub struct BbRoute {
	/// Tool name
	pub name: &'static str,
	/// URL path template with placeholders
	pub path: &'static str,
	/// Whether response is raw text (diff)
	pub is_text: bool,
	/// Whether to auto-paginate all pages
	pub is_list: bool,
}

/// Query param separator (? or &)
pub fn query_sep(path: &str) -> char {
	if path.contains('?') { '&' } else { '?' }
}

pub fn exec_bb_route(
	id: Value,
	_args: &Value,
	route: &BbRoute,
	client: &BbClient,
	path: &str,
) -> String {
	let result = if route.is_list {
		get_all_pages(client, path)
	} else {
		client.get(path)
	};
	match result {
		Ok(body) => wrap_text_content(id, &body),
		Err(err) => error_response(
			id, -32000, &err.to_string(),
		),
	}
}
