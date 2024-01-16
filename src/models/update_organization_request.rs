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
pub struct UpdateOrganizationRequest {
	/// Metadata saved on the organization, that is visible to both your frontend and backend.
	#[serde(rename = "public_metadata", skip_serializing_if = "Option::is_none")]
	pub public_metadata: Option<serde_json::Value>,
	/// Metadata saved on the organization that is only visible to your backend.
	#[serde(rename = "private_metadata", skip_serializing_if = "Option::is_none")]
	pub private_metadata: Option<serde_json::Value>,
	/// The new name of the organization
	#[serde(
		rename = "name",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub name: Option<Option<String>>,
	/// The new slug of the organization, which needs to be unique in the instance
	#[serde(
		rename = "slug",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub slug: Option<Option<String>>,
	/// The maximum number of memberships allowed for this organization
	#[serde(
		rename = "max_allowed_memberships",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub max_allowed_memberships: Option<Option<i64>>,
	/// If true, an admin can delete this organization with the Frontend API.
	#[serde(
		rename = "admin_delete_enabled",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub admin_delete_enabled: Option<Option<bool>>,
}

impl UpdateOrganizationRequest {
	pub fn new() -> UpdateOrganizationRequest {
		UpdateOrganizationRequest {
			public_metadata: None,
			private_metadata: None,
			name: None,
			slug: None,
			max_allowed_memberships: None,
			admin_delete_enabled: None,
		}
	}
}
