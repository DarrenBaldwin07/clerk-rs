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
pub struct RevokeInvitation200Response {
	#[serde(rename = "object")]
	pub object: Object,
	#[serde(rename = "id")]
	pub id: String,
	#[serde(rename = "email_address")]
	pub email_address: String,
	#[serde(rename = "public_metadata", skip_serializing_if = "Option::is_none")]
	pub public_metadata: Option<serde_json::Value>,
	#[serde(rename = "revoked", skip_serializing_if = "Option::is_none")]
	pub revoked: Option<bool>,
	#[serde(rename = "status")]
	pub status: Status,
	#[serde(
		rename = "url",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub url: Option<Option<String>>,
	/// Unix timestamp of creation.
	#[serde(rename = "created_at")]
	pub created_at: i64,
	/// Unix timestamp of last update.
	#[serde(rename = "updated_at")]
	pub updated_at: i64,
}

impl RevokeInvitation200Response {
	pub fn new(object: Object, id: String, email_address: String, status: Status, created_at: i64, updated_at: i64) -> RevokeInvitation200Response {
		RevokeInvitation200Response {
			object,
			id,
			email_address,
			public_metadata: None,
			revoked: None,
			status,
			url: None,
			created_at,
			updated_at,
		}
	}
}

///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
	#[serde(rename = "invitation")]
	Invitation,
}

impl Default for Object {
	fn default() -> Object {
		Self::Invitation
	}
}
///
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
	#[serde(rename = "revoked")]
	Revoked,
}

impl Default for Status {
	fn default() -> Status {
		Self::Revoked
	}
}
