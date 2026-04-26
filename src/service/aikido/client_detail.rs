use crate::domain::aikido::{
	IssueCounts, IssueCountFilters, IssueDetail,
};
use crate::domain::errors::aikido::AikidoResult;

use super::client::AikidoClient;
use super::client_request::push_filter;

/// Count and detail queries for Aikido issues
impl AikidoClient {
	/// Get issue counts by severity
	pub fn fetch_counts(
		&self,
		filters: &IssueCountFilters,
	) -> AikidoResult<IssueCounts> {
		let mut params = Vec::new();
		push_filter(
			&mut params,
			"filter_code_repo_name",
			&filters.repo_name,
		);
		if let Some(id) =
			filters.container_repo_id
		{
			params.push((
				"filter_container_repo_id",
				id.to_string(),
			));
		}
		self.get_json(
			"/public/v1/issues/counts",
			&params,
		)
	}

	/// Get single issue detail
	pub fn fetch_detail(
		&self,
		id: u64,
	) -> AikidoResult<IssueDetail> {
		let path =
			format!("/public/v1/issues/{id}");
		self.get_json(&path, &[])
	}
}
