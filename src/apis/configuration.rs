use crate::clerk::USER_AGENT;
use reqwest::header::{HeaderMap, AUTHORIZATION, USER_AGENT as REQWEST_USER_AGENT};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use async_trait::async_trait;

/*
 * Clerk configuration for constructing authenticated requests to the clerk.dev api
 *
 * Please refer to the clerk.dev official documentation for more information: https://docs.clerk.dev
 *
 */
/// OAuth2 token with expiration information
#[derive(Debug, Clone)]
pub struct OAuth2Token {
	pub access_token: String,
	pub token_type: String,
	pub refresh_token: Option<String>,
	pub expiry: Option<SystemTime>,
}

impl OAuth2Token {
	/// Create a new OAuth2Token
	pub fn new(
		access_token: String,
		token_type: String,
		refresh_token: Option<String>,
		expiry: Option<SystemTime>,
	) -> Self {
		Self {
			access_token,
			token_type,
			refresh_token,
			expiry,
		}
	}

	/// Check if the token is expired
	pub fn is_expired(&self) -> bool {
		if let Some(expiry) = self.expiry {
			// Consider the token expired if it's within 30 seconds of expiry
			let now = SystemTime::now();
			if let Ok(duration) = expiry.duration_since(now) {
				return duration <= Duration::from_secs(30);
			}
			return true;
		}
		false
	}

	/// Get the authorization header value
	pub fn authorization_header(&self) -> String {
		format!("{} {}", self.token_type, self.access_token)
	}
}

/// A trait for sources that can provide OAuth2 tokens
/// Similar to Go's TokenSource interface
#[async_trait]
pub trait OAuth2TokenSource: Send + Sync {
	/// Return a valid token or error
	async fn token(&self) -> Result<OAuth2Token, String>;
}

/// A source that always returns the same token
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
