use crate::{
	apis::configuration,
	endpoints::{ClerkDeleteEndpoint, ClerkGetEndpoint, ClerkPostEndpoint, ClerkPutEndpoint, ClerkDynamicGetEndpoint},
	util::generate_path_from_params
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
 * let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_owned()), None);
 * let client = Clerk::new(config);
 *
 * let res = client.get(ClerkGetEndpoint::GetUserList).await?;
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

	/// Make a PUT request to the specified Clerk API endpoint
	pub async fn patch<'a, T: Serialize + Deserialize<'a>>(
		&self,
		endpoint: ClerkPutEndpoint,
		body: T,
	) -> Result<serde_json::value::Value, reqwest::Error> {
		let parsed_endpoint = endpoint.to_string();
		let url = format!("{}{}", self.config.base_path, parsed_endpoint);

		match self.config.client.patch(&url).json(&body).send().await {
			Ok(response) => match response.json::<Value>().await {
				Ok(user) => Ok(user),
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}
	}

	pub async fn get_with_params(&self, endpoint: ClerkDynamicGetEndpoint, params: Vec<&str>) -> Result<serde_json::value::Value, reqwest::Error>  {
		let parsed_endpoint = endpoint.to_string();
		let url = format!("{}{}", self.config.base_path, parsed_endpoint);
		let url_with_params = generate_path_from_params(url, params);

		match self.config.client.get(&url_with_params).send().await {
			Ok(response) => match response.json::<Value>().await {
				Ok(user) => Ok(user),
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}
	}

	/// Make a POST request to the specified Clerk API endpoint
	pub async fn post_with_params<'a, T: Serialize + Deserialize<'a>>(
		&self,
		endpoint: ClerkPostEndpoint,
		body: T,
		params: Vec<&str>
	) -> Result<serde_json::value::Value, reqwest::Error> {
		let parsed_endpoint = endpoint.to_string();
		let url = format!("{}{}", self.config.base_path, parsed_endpoint);
		let url_with_params = generate_path_from_params(url, params);

		match self.config.client.post(&url_with_params).json(&body).send().await {
			Ok(response) => match response.json::<Value>().await {
				Ok(user) => Ok(user),
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}
	}


	/// Make a DELETE request to the specified Clerk API endpoint
	pub async fn delete_with_params(&self, endpoint: ClerkDeleteEndpoint, params: Vec<&str>) -> Result<serde_json::value::Value, reqwest::Error> {
		let parsed_endpoint = endpoint.to_string();
		let url = format!("{}{}", self.config.base_path, parsed_endpoint);
		let url_with_params = generate_path_from_params(url, params);

		match self.config.client.delete(&url_with_params).send().await {
			Ok(response) => match response.json::<Value>().await {
				Ok(user) => Ok(user),
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}
	}

	/// Make a PUT request to the specified Clerk API endpoint
	pub async fn put_with_params<'a, T: Serialize + Deserialize<'a>>(
		&self,
		endpoint: ClerkPutEndpoint,
		body: T,
		params: Vec<&str>
	) -> Result<serde_json::value::Value, reqwest::Error> {
		let parsed_endpoint = endpoint.to_string();
		let url = format!("{}{}", self.config.base_path, parsed_endpoint);
		let url_with_params = generate_path_from_params(url, params);

		match self.config.client.put(&url_with_params).json(&body).send().await {
			Ok(response) => match response.json::<Value>().await {
				Ok(user) => Ok(user),
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}
	}

	/// Make a PUT request to the specified Clerk API endpoint
	pub async fn patch_with_params<'a, T: Serialize + Deserialize<'a>>(
		&self,
		endpoint: ClerkPutEndpoint,
		body: T,
		params: Vec<&str>
	) -> Result<serde_json::value::Value, reqwest::Error> {
		let parsed_endpoint = endpoint.to_string();
		let url = format!("{}{}", self.config.base_path, parsed_endpoint);
		let url_with_params = generate_path_from_params(url, params);

		match self.config.client.patch(&url_with_params).json(&body).send().await {
			Ok(response) => match response.json::<Value>().await {
				Ok(user) => Ok(user),
				Err(e) => Err(e),
			},
			Err(e) => Err(e),
		}
	}
}
