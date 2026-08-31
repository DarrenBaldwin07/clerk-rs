use crate::clerk::{CLERK_API_VERSION, USER_AGENT};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT as USER_AGENT_HEADER};

/// Configuration used by every generated Clerk Backend API function.
#[derive(Debug, Clone)]
pub struct Configuration {
	pub base_path: String,
	pub user_agent: Option<String>,
	pub client: reqwest::Client,
	pub basic_auth: Option<BasicAuth>,
	pub oauth_access_token: Option<String>,
	pub bearer_access_token: Option<String>,
	pub api_key: Option<ApiKey>,
	pub api_version: Option<String>,
}

pub type BasicAuth = (String, Option<String>);

#[derive(Debug, Clone)]
pub struct ApiKey {
	pub prefix: Option<String>,
	pub key: String,
}

impl Configuration {
	/// Create an authenticated client targeting Clerk's current Backend API.
	pub fn new(secret_key: impl Into<String>) -> Self {
		Self {
			bearer_access_token: Some(secret_key.into()),
			..Self::default()
		}
	}

	/// Override the Backend API base URL, primarily for proxies and tests.
	pub fn with_base_path(mut self, base_path: impl Into<String>) -> Self {
		self.base_path = base_path.into();
		self
	}

	/// Select a dated Clerk API version and rebuild the default HTTP client.
	pub fn with_api_version(mut self, api_version: impl Into<String>) -> Self {
		self.api_version = Some(api_version.into());
		self.client = build_client(self.user_agent.as_deref(), self.api_version.as_deref());
		self
	}
}

impl Default for Configuration {
	fn default() -> Self {
		let user_agent = USER_AGENT.to_owned();
		let api_version = CLERK_API_VERSION.to_owned();

		Self {
			base_path: "https://api.clerk.com/v1".to_owned(),
			user_agent: Some(user_agent.clone()),
			client: build_client(Some(&user_agent), Some(&api_version)),
			basic_auth: None,
			oauth_access_token: None,
			bearer_access_token: None,
			api_key: None,
			api_version: Some(api_version),
		}
	}
}

fn build_client(user_agent: Option<&str>, api_version: Option<&str>) -> reqwest::Client {
	let mut headers = HeaderMap::new();
	if let Some(user_agent) = user_agent {
		headers.insert(
			USER_AGENT_HEADER,
			HeaderValue::from_str(user_agent).expect("the clerk-rs user agent is always a valid header value"),
		);
	}
	if let Some(api_version) = api_version {
		headers.insert(
			"Clerk-API-Version",
			HeaderValue::from_str(api_version).expect("the Clerk API version must be a valid header value"),
		);
	}

	reqwest::Client::builder()
		.default_headers(headers)
		.build()
		.expect("failed to initialize the Clerk HTTP client")
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn authenticated_configuration_targets_the_pinned_api() {
		let configuration = Configuration::new("sk_test_example");

		assert_eq!(configuration.base_path, "https://api.clerk.com/v1");
		assert_eq!(configuration.api_version.as_deref(), Some(CLERK_API_VERSION));
		assert_eq!(configuration.bearer_access_token.as_deref(), Some("sk_test_example"));
	}
}
