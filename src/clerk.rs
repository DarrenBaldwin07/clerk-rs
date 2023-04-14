use crate::{
	apis::configuration,
	endpoints::{ClerkDeleteEndpoint, ClerkGetEndpoint, ClerkPostEndpoint, ClerkPutEndpoint},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// Default user agent used for clerk-rs sdk (this is sent with every clerk api request)
pub const USER_AGENT: &str = concat!("Clerk/v1 RustBindings/", env!("CARGO_PKG_VERSION"));

/*
 * Unofficial Clerk SDK
 *
 * Please refer to the clerk.dev official documentation for more information: https://docs.clerk.dev
 *
 * # Examples
 * ```
 * use clerk_rs::Clerk;
 * use clerk_rs::apis::configuration::ClerkConfiguration;
 *
 *
 * ```
 *
 * NOTE: This SDK is based on the official clerk openAPI spec found here: https://clerk.com/docs/reference/backend-api
 */
pub struct Clerk {
	pub config: configuration::ClerkConfiguration,
}

impl Clerk {
	// Creates a new Clerk SDK client for making requests out to the public Clerk api:
	pub fn new(clerk_configuration: configuration::ClerkConfiguration) -> Self {
		Self { config: clerk_configuration }
	}

	/// Make a GET request to the specified Clerk API endpoint
	pub async fn get(&self, endpoint: ClerkGetEndpoint) -> Result<serde_json::value::Value, reqwest::Error> {
		let parsed_endpoint = endpoint.to_string();
		let url = format!("{}{}", self.config.base_path, parsed_endpoint);

		match self.config.client.get(&url).send().await {
			Ok(response) => match response.json::<Value>().await {
				Ok(user) => Ok(user),
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}
	}

	/// Make a POST request to the specified Clerk API endpoint
	pub async fn post<'a, T: Serialize + Deserialize<'a>>(
		&self,
		endpoint: ClerkPostEndpoint,
		body: T,
	) -> Result<serde_json::value::Value, reqwest::Error> {
		let parsed_endpoint = endpoint.to_string();
		let url = format!("{}{}", self.config.base_path, parsed_endpoint);

		match self.config.client.post(&url).json(&body).send().await {
			Ok(response) => match response.json::<Value>().await {
				Ok(user) => Ok(user),
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}
	}

	/// Make a DELETE request to the specified Clerk API endpoint
	pub async fn delete(&self, endpoint: ClerkDeleteEndpoint) -> Result<serde_json::value::Value, reqwest::Error> {
		let parsed_endpoint = endpoint.to_string();
		let url = format!("{}{}", self.config.base_path, parsed_endpoint);

		match self.config.client.delete(&url).send().await {
			Ok(response) => match response.json::<Value>().await {
				Ok(user) => Ok(user),
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}
	}

	/// Make a PUT request to the specified Clerk API endpoint
	pub async fn put<'a, T: Serialize + Deserialize<'a>>(
		&self,
		endpoint: ClerkPutEndpoint,
		body: T,
	) -> Result<serde_json::value::Value, reqwest::Error> {
		let parsed_endpoint = endpoint.to_string();
		let url = format!("{}{}", self.config.base_path, parsed_endpoint);

		match self.config.client.put(&url).json(&body).send().await {
			Ok(response) => match response.json::<Value>().await {
				Ok(user) => Ok(user),
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}
	}

	// TODO: support methods for all http methods but with dynamic params
}
