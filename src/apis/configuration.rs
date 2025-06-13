use crate::clerk::USER_AGENT;
use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT as REQWEST_USER_AGENT};
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};
use async_trait::async_trait;

/*
 * Clerk configuration for constructing authenticated requests to the clerk.dev api
 *
 * Please refer to the clerk.dev official documentation for more information: https://docs.clerk.dev
 *
 */
#[derive(Debug, Clone)]
pub struct ClerkConfiguration {
	pub base_path: String,
	pub user_agent: Option<String>,
	pub client: reqwest::Client,
	pub basic_auth: Option<BasicAuth>,
	pub oauth_access_token: Option<String>,
	pub bearer_access_token: Option<String>,
	pub api_key: Option<ApiKey>,
	pub oauth2_token_source: Option<Arc<dyn OAuth2TokenSource>>,
}

/// Merged auth allowing user to pass in a bearer token AND a api_key etc
pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
	pub prefix: Option<String>,
	pub key: String,
}

/// OAuth2 token represents an OAuth 2.0 token with its expiry time
#[derive(Debug, Clone)]
pub struct OAuth2Token {
	pub access_token: String,
	pub token_type: String,
	pub refresh_token: Option<String>,
	pub expiry: Option<SystemTime>,
}

impl OAuth2Token {
	/// Returns true if the token is expired or will expire within the given duration
	pub fn will_expire_in(&self, duration: Duration) -> bool {
		if let Some(expiry) = self.expiry {
			if let Ok(duration_until_expiry) = expiry.duration_since(SystemTime::now()) {
				return duration_until_expiry <= duration;
			}
			return true; // If we can't calculate duration, assume it's expired
		}
		false // No expiry means it doesn't expire
	}

	/// Returns true if the token is expired
	pub fn is_expired(&self) -> bool {
		if let Some(expiry) = self.expiry {
			if let Ok(_) = SystemTime::now().duration_since(expiry) {
				return true;
			}
		}
		false
	}

	/// Returns the token prefixed with the token type (e.g., "Bearer TOKEN")
	pub fn token_with_type(&self) -> String {
		format!("{} {}", self.token_type, self.access_token)
	}
}

/// OAuth2TokenSource provides a way to obtain OAuth 2.0 tokens
#[async_trait]
pub trait OAuth2TokenSource: Send + Sync {
	/// Returns a valid OAuth 2.0 token
	async fn token(&self) -> Result<OAuth2Token, String>;
}

/// StaticTokenSource returns a static token
#[derive(Debug, Clone)]
pub struct StaticTokenSource {
	token: OAuth2Token,
}

impl StaticTokenSource {
	pub fn new(token: OAuth2Token) -> Self {
		Self { token }
	}
}

#[async_trait]
impl OAuth2TokenSource for StaticTokenSource {
	async fn token(&self) -> Result<OAuth2Token, String> {
		Ok(self.token.clone())
	}
}

/// CachedTokenSource caches tokens from another token source
#[derive(Debug)]
pub struct CachedTokenSource {
	source: Arc<dyn OAuth2TokenSource>,
	cached_token: Mutex<Option<OAuth2Token>>,
}

impl CachedTokenSource {
	pub fn new(source: Arc<dyn OAuth2TokenSource>) -> Self {
		Self {
			source,
			cached_token: Mutex::new(None),
		}
	}
}

impl Clone for CachedTokenSource {
	fn clone(&self) -> Self {
		Self {
			source: self.source.clone(),
			cached_token: Mutex::new(None),
		}
	}
}

#[async_trait]
impl OAuth2TokenSource for CachedTokenSource {
	async fn token(&self) -> Result<OAuth2Token, String> {
		// Check if we have a cached token that's still valid
		let cached_token_expired = {
			let cached_token_guard = self.cached_token.lock().unwrap();
			if let Some(token) = &*cached_token_guard {
				token.is_expired()
			} else {
				true
			}
		};

		if cached_token_expired {
			// Get a new token
			let new_token = self.source.token().await?;
			
			// Update the cached token
			let mut cached_token_guard = self.cached_token.lock().unwrap();
			*cached_token_guard = Some(new_token.clone());
			return Ok(new_token);
		}

		// Return the cached token
		let cached_token_guard = self.cached_token.lock().unwrap();
		Ok(cached_token_guard.as_ref().unwrap().clone())
	}
}

impl ClerkConfiguration {
	// Creates a new client ClerkConfiguration object used to authenticate requests to the clerk.dev api
	pub fn new(
		basic_auth: Option<BasicAuth>,
		oauth_access_token: Option<String>,
		bearer_access_token: Option<String>,
		api_key: Option<ApiKey>,
		oauth2_token_source: Option<Arc<dyn OAuth2TokenSource>>,
	) -> Self {
		// Generate our auth token
		let construct_bearer_token = format!("Bearer {}", bearer_access_token.as_ref().unwrap_or(&String::from("")));
		// Initialize our Clerk SDK with the default user_agent and auth headers
		let mut headers = HeaderMap::new();
		headers.insert(REQWEST_USER_AGENT, USER_AGENT.parse().unwrap());
		headers.insert(
			AUTHORIZATION,
			construct_bearer_token
				.parse()
				.expect("Error: could not parse Bearer auth token into a valid request header."),
		);

		// Construct our http client (we should also support hyper client instead of reqwest in the future)
		let client = reqwest::Client::builder()
			.default_headers(headers)
			.build()
			.expect("Error: could not initialize Clerk SDK client. Please try again!");

		Self {
			base_path: "https://api.clerk.dev/v1".to_owned(),
			user_agent: Some(USER_AGENT.to_owned()),
			client,
			basic_auth,
			oauth_access_token,
			bearer_access_token,
			api_key,
			oauth2_token_source,
		}
	}
}

impl Default for ClerkConfiguration {
	fn default() -> Self {
		// Initialize our Clerk SDK with the default user_agent and all stock settings (this will only give the user access to a select few clerk apis that are usable without full authorization)
		let mut headers = HeaderMap::new();
		headers.insert(REQWEST_USER_AGENT, USER_AGENT.parse().unwrap());
		let client = reqwest::Client::builder()
			.default_headers(headers)
			.build()
			.expect("Error: could not initialize Clerk SDK client. Please try again!");

		Self {
			base_path: "https://api.clerk.dev/v1".to_owned(),
			user_agent: Some(USER_AGENT.to_owned()),
			client,
			basic_auth: None,
			oauth_access_token: None,
			bearer_access_token: None,
			api_key: None,
			oauth2_token_source: None,
		}
	}
}
