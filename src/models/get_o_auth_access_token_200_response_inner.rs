/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 * Generated by: https://openapi-generator.tech
 */

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct GetOAuthAccessToken200ResponseInner {
	#[serde(rename = "object", skip_serializing_if = "Option::is_none")]
	pub object: Option<String>,
	/// The access token
	#[serde(rename = "token", skip_serializing_if = "Option::is_none")]
	pub token: Option<String>,
	/// The ID of the provider
	#[serde(rename = "provider", skip_serializing_if = "Option::is_none")]
	pub provider: Option<String>,
	#[serde(rename = "public_metadata", skip_serializing_if = "Option::is_none")]
	pub public_metadata: Option<serde_json::Value>,
	#[serde(
		rename = "label",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub label: Option<Option<String>>,
	/// The list of scopes that the token is valid for. Only present for OAuth 2.0 tokens.
	#[serde(rename = "scopes", skip_serializing_if = "Option::is_none")]
	pub scopes: Option<Vec<String>>,
	/// The token secret. Only present for OAuth 1.0 tokens.
	#[serde(rename = "token_secret", skip_serializing_if = "Option::is_none")]
	pub token_secret: Option<String>,
}

impl GetOAuthAccessToken200ResponseInner {
	pub fn new() -> GetOAuthAccessToken200ResponseInner {
		GetOAuthAccessToken200ResponseInner {
			object: None,
			token: None,
			provider: None,
			public_metadata: None,
			label: None,
			scopes: None,
			token_secret: None,
		}
	}
}
