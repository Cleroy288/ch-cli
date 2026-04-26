use reqwest::blocking::Client;

use crate::domain::errors::atlassian::{
	AtlassianError, AtlassianResult,
};

use super::types::BbCredentials;

/// Base URL for Bitbucket API
const BB_API_BASE: &str =
	"https://api.bitbucket.org/2.0";

/// Blocking Bitbucket API client
pub struct BbClient {
	http: Client,
	creds: BbCredentials,
}

impl BbClient {
	pub fn new(creds: BbCredentials) -> Self {
		Self {
			http: Client::new(),
			creds,
		}
	}

	/// GET JSON from a Bitbucket API path
	pub fn get(
		&self,
		path: &str,
	) -> AtlassianResult<String> {
		let url =
			format!("{BB_API_BASE}{path}");
		self.fetch(&url)
	}

	/// GET by full URL (for pagination `next` links)
	pub fn fetch_url(
		&self,
		url: &str,
	) -> AtlassianResult<String> {
		self.fetch(url)
	}

	fn fetch(
		&self,
		url: &str,
	) -> AtlassianResult<String> {
		let resp = self
			.http
			.get(url)
			.basic_auth(
				&self.creds.email,
				Some(&self.creds.token),
			)
			.send()
			.map_err(|e| {
				AtlassianError::Http(e.to_string())
			})?;
		let status = resp.status();
		if !status.is_success() {
			return Err(AtlassianError::Status(
				format!("Bitbucket API: {status}"),
			));
		}
		resp.text().map_err(|e| {
			AtlassianError::Http(e.to_string())
		})
	}
}
