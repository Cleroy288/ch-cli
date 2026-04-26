use std::sync::Mutex;
use std::time::{Duration, Instant};

use base64::Engine;
use reqwest::blocking::Client;

use crate::domain::errors::aikido::{
	AikidoError, AikidoResult,
};

/// Token refresh margin (seconds)
const TOKEN_MARGIN: Duration =
	Duration::from_secs(60);

/// OAuth2 token endpoint
const TOKEN_URL: &str =
	"https://app.aikido.dev/api/oauth/token";

/// OAuth2 grant body
const OAUTH_GRANT_BODY: &str =
	"grant_type=client_credentials";

/// Cached OAuth2 access token
struct TokenCache {
	access_token: String,
	expires_at: Instant,
}

/// Token response from Aikido OAuth2
#[derive(serde::Deserialize)]
struct TokenResponse {
	access_token: String,
	expires_in: u64,
}

/// OAuth2 token manager (blocking)
pub struct AuthManager {
	http: Client,
	client_id: String,
	client_secret: String,
	cache: Mutex<Option<TokenCache>>,
}

impl AuthManager {
	pub fn new(
		http: Client,
		client_id: String,
		client_secret: String,
	) -> Self {
		Self {
			http,
			client_id,
			client_secret,
			cache: Mutex::new(None),
		}
	}

	/// Returns a valid bearer token
	pub fn token(
		&self,
	) -> AikidoResult<String> {
		let mut guard = self
			.cache
			.lock()
			.map_err(|e| {
				AikidoError::Auth(e.to_string())
			})?;
		if let Some(cached) = guard.as_ref() {
			if Instant::now() < cached.expires_at {
				return Ok(
					cached.access_token.clone()
				);
			}
		}
		let (cache, access_token) =
			self.fetch_token()?;
		*guard = Some(cache);
		Ok(access_token)
	}

	fn fetch_token(
		&self,
	) -> AikidoResult<(TokenCache, String)> {
		let encoded = base64::engine
			::general_purpose::STANDARD
			.encode(format!(
				"{}:{}",
				self.client_id, self.client_secret,
			));
		let resp: TokenResponse = self
			.http
			.post(TOKEN_URL)
			.header(
				"Authorization",
				format!("Basic {encoded}"),
			)
			.header(
				"Content-Type",
				"application/x-www-form-urlencoded",
			)
			.body(OAUTH_GRANT_BODY)
			.send()
			.map_err(|e| {
				AikidoError::Auth(e.to_string())
			})?
			.error_for_status()
			.map_err(|e| {
				AikidoError::Auth(e.to_string())
			})?
			.json()
			.map_err(|e| {
				AikidoError::Json(e.to_string())
			})?;
		let expires_at = Instant::now()
			+ Duration::from_secs(resp.expires_in)
				.saturating_sub(TOKEN_MARGIN);
		let token = resp.access_token.clone();
		let cache = TokenCache {
			access_token: resp.access_token,
			expires_at,
		};
		Ok((cache, token))
	}
}
