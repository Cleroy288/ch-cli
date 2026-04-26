use serde::de::DeserializeOwned;

use crate::domain::errors::aikido::{
	AikidoError, AikidoResult,
};

use super::client::AikidoClient;

/// Max items per page
const PAGE_SIZE: u32 = 20;

/// Base URL for Aikido REST API
pub const BASE_URL: &str =
	"https://app.aikido.dev/api";

/// Authenticated GET returning parsed JSON
impl AikidoClient {
	pub(super) fn get_json<T: DeserializeOwned>(
		&self,
		path: &str,
		params: &[(&str, String)],
	) -> AikidoResult<T> {
		let token = self.auth.token()?;
		let pairs: Vec<(&str, &str)> = params
			.iter()
			.map(|(k, v)| (*k, v.as_str()))
			.collect();
		let url = format!("{BASE_URL}{path}");
		let resp = self
			.http
			.get(&url)
			.bearer_auth(&token)
			.query(&pairs)
			.send()
			.map_err(|e| {
				AikidoError::Http(e.to_string())
			})?;
		check_status(&resp)?;
		resp.json().map_err(|e| {
			AikidoError::Json(e.to_string())
		})
	}

	pub(super) fn paginate<T: DeserializeOwned>(
		&self,
		path: &str,
	) -> AikidoResult<Vec<T>> {
		let mut all = Vec::new();
		let mut page: u32 = 0;
		let size = PAGE_SIZE.to_string();
		loop {
			let pg = page.to_string();
			let params = vec![
				("page", pg),
				("per_page", size.clone()),
			];
			let batch: Vec<T> =
				self.get_json(path, &params)?;
			let done =
				batch.len() < PAGE_SIZE as usize;
			all.extend(batch);
			if done {
				break;
			}
			page += 1;
		}
		Ok(all)
	}
}

/// Append optional filter to query params
pub fn push_filter<'a>(
	params: &mut Vec<(&'a str, String)>,
	key: &'a str,
	value: &Option<String>,
) {
	if let Some(v) = value {
		params.push((key, v.clone()));
	}
}

fn check_status(
	resp: &reqwest::blocking::Response,
) -> AikidoResult<()> {
	let status = resp.status();
	if !status.is_success() {
		return Err(AikidoError::Status(
			format!("Aikido API: {status}"),
		));
	}
	Ok(())
}
