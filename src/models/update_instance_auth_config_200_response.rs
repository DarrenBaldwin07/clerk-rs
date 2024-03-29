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
pub struct UpdateInstanceAuthConfig200Response {
	/// String representing the object's type. Objects of the same type share the same value.
	#[serde(rename = "object", skip_serializing_if = "Option::is_none")]
	pub object: Option<Object>,
	#[serde(rename = "id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	#[serde(rename = "restricted_to_allowlist", skip_serializing_if = "Option::is_none")]
	pub restricted_to_allowlist: Option<bool>,
	#[serde(rename = "from_email_address", skip_serializing_if = "Option::is_none")]
	pub from_email_address: Option<String>,
	#[serde(rename = "progressive_sign_up", skip_serializing_if = "Option::is_none")]
	pub progressive_sign_up: Option<bool>,
	#[serde(rename = "enhanced_email_deliverability", skip_serializing_if = "Option::is_none")]
	pub enhanced_email_deliverability: Option<bool>,
}

impl UpdateInstanceAuthConfig200Response {
	pub fn new() -> UpdateInstanceAuthConfig200Response {
		UpdateInstanceAuthConfig200Response {
			object: None,
			id: None,
			restricted_to_allowlist: None,
			from_email_address: None,
			progressive_sign_up: None,
			enhanced_email_deliverability: None,
		}
	}
}

/// String representing the object's type. Objects of the same type share the same value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
	#[serde(rename = "instance_settings")]
	InstanceSettings,
}

impl Default for Object {
	fn default() -> Object {
		Self::InstanceSettings
	}
}
