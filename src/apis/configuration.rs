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
pub struct Token {
	pub access_token: String,
	pub token_type: String,
	pub refresh_token: Option<String>,
	pub expiry: Option<SystemTime>,
}

impl Token {
	pub fn new(access_token: String, token_type: String, refresh_token: Option<String>, expiry: Option<SystemTime>) -> Self {
		Self {
			access_token,
			token_type,
			refresh_token,
			expiry,
		}
	}

	pub fn expired(&self) -> bool {
		if let Some(expiry) = self.expiry {
			// Consider the token expired slightly before its actual expiration
			// to account for clock skew and request time
			if let Ok(now_plus_leeway) = SystemTime::now().checked_add(Duration::from_secs(30)) {
				return expiry <= now_plus_leeway;
			}
		}
		false
	}

	pub fn valid(&self) -> bool {
		!self.access_token.is_empty() && !self.expired()
	}
}

#[async_trait]
pub trait TokenSource: Send + Sync {
	async fn token(&self) -> Result<Token, String>;
}

#[derive(Debug, Clone)]
pub struct StaticTokenSource {
	pub token: Token,
}

impl StaticTokenSource {
	pub fn new(access_token: String) -> Self {
		Self {
			token: Token::new(access_token, "Bearer".to_string(), None, None),
		}
	}
}

#[async_trait]
impl TokenSource for StaticTokenSource {
	async fn token(&self) -> Result<Token, String> {
		Ok(self.token.clone())
	}
}

#[derive(Debug, Clone)]
pub struct ReuseTokenSource {
	new_source: Arc<dyn TokenSource>,
	current_token: Arc<Mutex<Option<Token>>>,
}

impl ReuseTokenSource {
	pub fn new(new_source: Arc<dyn TokenSource>, initial_token: Option<Token>) -> Self {
		Self {
			new_source,
			current_token: Arc::new(Mutex::new(initial_token)),
		}
	}
}

#[async_trait]
impl TokenSource for ReuseTokenSource {
	async fn token(&self) -> Result<Token, String> {
		let mut token_guard = self.current_token.lock().map_err(|e| e.to_string())?;
		
		if let Some(token) = token_guard.as_ref() {
			if token.valid() {
				return Ok(token.clone());
			}
		}
		
		// Get a new token
		let new_token = self.new_source.token().await?;
		*token_guard = Some(new_token.clone());
		
		Ok(new_token)
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
	pub token_source: Option<Arc<dyn TokenSource>>,
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
		token_source: Option<Arc<dyn TokenSource>>,
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
			token_source,
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
			token_source: None,
		}
	}
}
