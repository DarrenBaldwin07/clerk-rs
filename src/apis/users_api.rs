/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

use reqwest;

use super::Error;
use crate::{apis::ResponseContent, clerk::Clerk};

/// struct for typed errors of method [`ban_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BanUserError {
	Status402(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUserError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status422(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`disable_mfa`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DisableMfaError {
	Status404(crate::models::ClerkErrors),
	Status500(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_o_auth_access_token`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetOAuthAccessTokenError {
	Status422(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_list`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserListError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status422(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_users_count`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUsersCountError {
	Status422(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unban_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnbanUserError {
	Status402(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	Status422(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_user_metadata`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserMetadataError {
	Status400(crate::models::ClerkErrors),
	Status401(crate::models::ClerkErrors),
	Status404(crate::models::ClerkErrors),
	Status422(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`users_get_organization_memberships`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UsersGetOrganizationMembershipsError {
	Status403(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`verify_password`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyPasswordError {
	Status400(),
	Status404(),
	Status422(),
	Status500(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`verify_totp`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum VerifyTotpError {
	Status400(),
	Status404(),
	Status422(),
	Status500(crate::models::ClerkErrors),
	UnknownValue(serde_json::Value),
}

pub struct User;

impl User {
	/// Marks the given user as banned, which means that all their sessions are revoked and they are not allowed to sign in again.
	pub async fn ban_user(clerk_client: &Clerk, user_id: &str) -> Result<crate::models::User, Error<BanUserError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/users/{user_id}/ban",
			local_var_configuration.base_path,
			user_id = crate::apis::urlencode(user_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<BanUserError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Creates a new user. Your user management settings determine how you should setup your user model.  Any email address and phone number created using this method will be marked as verified.  Note: If you are performing a migration, check out our guide on [zero downtime migrations](https://clerk.com/docs/deployments/import-users).  A rate limit rule of 20 requests per 10 seconds is applied to this endpoint.
	pub async fn create_user(
		clerk_client: &Clerk,
		create_user_request: crate::models::CreateUserRequest,
	) -> Result<crate::models::User, Error<CreateUserError>> {
		let local_var_configuration = &clerk_client.config;


		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!("{}/users", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		local_var_req_builder = local_var_req_builder.json(&create_user_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<CreateUserError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Delete the specified user
	pub async fn delete_user(clerk_client: &Clerk, user_id: &str) -> Result<crate::models::DeletedObject, Error<DeleteUserError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/users/{user_id}",
			local_var_configuration.base_path,
			user_id = crate::apis::urlencode(user_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<DeleteUserError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Disable all of a user's MFA methods (e.g. OTP sent via SMS, TOTP on their authenticator app) at once.
	pub async fn disable_mfa(clerk_client: &Clerk, user_id: &str) -> Result<crate::models::DisableMfa200Response, Error<DisableMfaError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/users/{user_id}/mfa",
			local_var_configuration.base_path,
			user_id = crate::apis::urlencode(user_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<DisableMfaError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Fetch the corresponding OAuth access token for a user that has previously authenticated with a particular OAuth provider. For OAuth 2.0, if the access token has expired and we have a corresponding refresh token, the access token will be refreshed transparently the new one will be returned.
	pub async fn get_o_auth_access_token(
		clerk_client: &Clerk,
		user_id: &str,
		provider: &str,
	) -> Result<Vec<crate::models::GetOAuthAccessToken200ResponseInner>, Error<GetOAuthAccessTokenError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/users/{user_id}/oauth_access_tokens/{provider}",
			local_var_configuration.base_path,
			user_id = crate::apis::urlencode(user_id),
			provider = crate::apis::urlencode(provider)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<GetOAuthAccessTokenError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Retrieve the details of a user
	pub async fn get_user(clerk_client: &Clerk, user_id: &str) -> Result<crate::models::User, Error<GetUserError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/users/{user_id}",
			local_var_configuration.base_path,
			user_id = crate::apis::urlencode(user_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<GetUserError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Returns a list of all users. The users are returned sorted by creation date, with the newest users appearing first.
	pub async fn get_user_list(
		clerk_client: &Clerk,
		email_address: Option<Vec<String>>,
		phone_number: Option<Vec<String>>,
		external_id: Option<Vec<String>>,
		username: Option<Vec<String>>,
		web3_wallet: Option<Vec<String>>,
		user_id: Option<Vec<String>>,
		organization_id: Option<Vec<String>>,
		query: Option<&str>,
		limit: Option<u64>,
		offset: Option<u64>,
		order_by: Option<&str>,
	) -> Result<Vec<crate::models::User>, Error<GetUserListError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!("{}/users", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(local_var_str) = email_address {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("email_address", p)).collect::<Vec<(&str, String)>>()),
				_ => local_var_req_builder.query(&[("email_address", &local_var_str.join(",").to_string())]),
			};
		}
		if let Some(local_var_str) = phone_number {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("phone_number", p)).collect::<Vec<(&str, String)>>()),
				_ => local_var_req_builder.query(&[("phone_number", &local_var_str.join(",").to_string())]),
			};
		}
		if let Some(local_var_str) = external_id {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("external_id", p)).collect::<Vec<(&str, String)>>()),
				_ => local_var_req_builder.query(&[("external_id", &local_var_str.join(",").to_string())]),
			};
		}
		if let Some(local_var_str) = username {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("username", p)).collect::<Vec<(&str, String)>>()),
				_ => local_var_req_builder.query(&[("username", &local_var_str.join(",").to_string())]),
			};
		}
		if let Some(local_var_str) = web3_wallet {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("web3_wallet", p)).collect::<Vec<(&str, String)>>()),
				_ => local_var_req_builder.query(&[("web3_wallet", &local_var_str.join(",").to_string())]),
			};
		}
		if let Some(local_var_str) = user_id {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("user_id", p)).collect::<Vec<(&str, String)>>()),
				_ => local_var_req_builder.query(&[(
					"user_id",
					&local_var_str
						.into_iter()
						.map(|p| p.to_string())
						.collect::<Vec<String>>()
						.join(",")
						.to_string(),
				)]),
			};
		}
		if let Some(local_var_str) = organization_id {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("organization_id", p)).collect::<Vec<(&str, String)>>()),
				_ => local_var_req_builder.query(&[("organization_id", &local_var_str.join(",").to_string())]),
			};
		}
		if let Some(local_var_str) = query {
			local_var_req_builder = local_var_req_builder.query(&[("query", local_var_str)]);
		}
		if let Some(local_var_str) = limit {
			local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
		}
		if let Some(local_var_str) = offset {
			local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
		}
		if let Some(local_var_str) = order_by {
			local_var_req_builder = local_var_req_builder.query(&[("order_by", local_var_str)]);
		}
		if let Some(local_var_user_agent) = local_var_configuration.user_agent.as_ref() {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<GetUserListError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Returns a total count of all users that match the given filtering criteria.
	pub async fn get_users_count(
		clerk_client: &Clerk,
		email_address: Option<Vec<String>>,
		phone_number: Option<Vec<String>>,
		external_id: Option<Vec<String>>,
		username: Option<Vec<String>>,
		web3_wallet: Option<Vec<String>>,
		user_id: Option<Vec<String>>,
		query: Option<&str>,
	) -> Result<crate::models::TotalCount, Error<GetUsersCountError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!("{}/users/count", local_var_configuration.base_path);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(local_var_str) = email_address {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("email_address", p)).collect::<Vec<(&str, String)>>()),
				_ => local_var_req_builder.query(&[("email_address", &local_var_str.join(",").to_string())]),
			};
		}
		if let Some(local_var_str) = phone_number {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("phone_number", p)).collect::<Vec<(&str, String)>>()),
				_ => local_var_req_builder.query(&[(
					"phone_number",
					&local_var_str
						.into_iter()
						.map(|p| p.to_string())
						.collect::<Vec<String>>()
						.join(",")
						.to_string(),
				)]),
			};
		}
		if let Some(local_var_str) = external_id {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("external_id", p)).collect::<Vec<(&str, String)>>()),
				_ => local_var_req_builder.query(&[("external_id", &local_var_str.join(",").to_string())]),
			};
		}
		if let Some(local_var_str) = username {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("username", p)).collect::<Vec<(&str, String)>>()),
				_ => local_var_req_builder.query(&[("username", &local_var_str.join(",").to_string())]),
			};
		}
		if let Some(local_var_str) = web3_wallet {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("web3_wallet", p)).collect::<Vec<(&str, String)>>()),
				_ => local_var_req_builder.query(&[("web3_wallet", &local_var_str.join(",").to_string())]),
			};
		}
		if let Some(local_var_str) = user_id {
			local_var_req_builder = match "multi" {
				"multi" => local_var_req_builder.query(&local_var_str.into_iter().map(|p| ("user_id", p)).collect::<Vec<(&str, String)>>()),
				_ => local_var_req_builder.query(&[("user_id", &local_var_str.join(",").to_string())]),
			};
		}
		if let Some(local_var_str) = query {
			local_var_req_builder = local_var_req_builder.query(&[("query", local_var_str)]);
		}
		if let Some(local_var_user_agent) = &local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent);
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<GetUsersCountError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Removes the ban mark from the given user.
	pub async fn unban_user(clerk_client: &Clerk, user_id: &str) -> Result<crate::models::User, Error<UnbanUserError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/users/{user_id}/unban",
			local_var_configuration.base_path,
			user_id = crate::apis::urlencode(user_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UnbanUserError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Update a user's attributes.  You can set the user's primary contact identifiers (email address and phone numbers) by updating the `primary_email_address_id` and `primary_phone_number_id` attributes respectively. Both IDs should correspond to verified identifications that belong to the user.  You can remove a user's username by setting the username attribute to null or the blank string \"\". This is a destructive action; the identification will be deleted forever. Usernames can be removed only if they are optional in your instance settings and there's at least one other identifier which can be used for authentication.
	pub async fn update_user(
		clerk_client: &Clerk,
		user_id: &str,
		update_user_request: crate::models::UpdateUserRequest,
	) -> Result<crate::models::User, Error<UpdateUserError>> {
		let local_var_configuration = &clerk_client.config;


		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/users/{user_id}",
			local_var_configuration.base_path,
			user_id = crate::apis::urlencode(user_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		local_var_req_builder = local_var_req_builder.json(&update_user_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UpdateUserError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Update a user's metadata attributes by merging existing values with the provided parameters.  This endpoint behaves differently than the *Update a user* endpoint. Metadata values will not be replaced entirely. Instead, a deep merge will be performed. Deep means that any nested JSON objects will be merged as well.  You can remove metadata keys at any level by setting their value to `null`.
	pub async fn update_user_metadata(
		clerk_client: &Clerk,
		user_id: &str,
		update_user_metadata_request: Option<crate::models::UpdateUserMetadataRequest>,
	) -> Result<crate::models::User, Error<UpdateUserMetadataError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/users/{user_id}/metadata",
			local_var_configuration.base_path,
			user_id = crate::apis::urlencode(user_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		local_var_req_builder = local_var_req_builder.json(&update_user_metadata_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UpdateUserMetadataError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Retrieve a paginated list of the user's organization memberships
	pub async fn users_get_organization_memberships(
		clerk_client: &Clerk,
		user_id: &str,
		limit: Option<u64>,
		offset: Option<u64>,
	) -> Result<crate::models::OrganizationMemberships, Error<UsersGetOrganizationMembershipsError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/users/{user_id}/organization_memberships",
			local_var_configuration.base_path,
			user_id = crate::apis::urlencode(user_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

		if let Some(ref local_var_str) = limit {
			local_var_req_builder = local_var_req_builder.query(&[("limit", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_str) = offset {
			local_var_req_builder = local_var_req_builder.query(&[("offset", &local_var_str.to_string())]);
		}
		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<UsersGetOrganizationMembershipsError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Check that the user's password matches the supplied input. Useful for custom auth flows and re-verification.
	pub async fn verify_password(
		clerk_client: &Clerk,
		user_id: &str,
		verify_password_request: Option<crate::models::VerifyPasswordRequest>,
	) -> Result<crate::models::VerifyPassword200Response, Error<VerifyPasswordError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/users/{user_id}/verify_password",
			local_var_configuration.base_path,
			user_id = crate::apis::urlencode(user_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		local_var_req_builder = local_var_req_builder.json(&verify_password_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<VerifyPasswordError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}

	/// Verify that the provided TOTP or backup code is valid for the user. Verifying a backup code will result it in being consumed (i.e. it will become invalid). Useful for custom auth flows and re-verification.
	pub async fn verify_totp(
		clerk_client: &Clerk,
		user_id: &str,
		verify_totp_request: Option<crate::models::VerifyTotpRequest>,
	) -> Result<crate::models::VerifyTotp200Response, Error<VerifyTotpError>> {
		let local_var_configuration = &clerk_client.config;

		let local_var_client = &local_var_configuration.client;

		let local_var_uri_str = format!(
			"{}/users/{user_id}/verify_totp",
			local_var_configuration.base_path,
			user_id = crate::apis::urlencode(user_id)
		);
		let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

		if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
			local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
		}

		local_var_req_builder = local_var_req_builder.json(&verify_totp_request);

		let local_var_req = local_var_req_builder.build()?;
		let local_var_resp = local_var_client.execute(local_var_req).await?;

		let local_var_status = local_var_resp.status();
		let local_var_content = local_var_resp.text().await?;

		if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
			serde_json::from_str(&local_var_content).map_err(Error::from)
		} else {
			let local_var_entity: Option<VerifyTotpError> = serde_json::from_str(&local_var_content).ok();
			let local_var_error = ResponseContent {
				status: local_var_status,
				content: local_var_content,
				entity: local_var_entity,
			};
			Err(Error::ResponseError(local_var_error))
		}
	}
}
