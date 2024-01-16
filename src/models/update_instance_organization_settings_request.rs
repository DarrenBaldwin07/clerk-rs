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
pub struct UpdateInstanceOrganizationSettingsRequest {
	#[serde(
		rename = "enabled",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub enabled: Option<Option<bool>>,
	#[serde(
		rename = "max_allowed_memberships",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub max_allowed_memberships: Option<Option<i64>>,
	#[serde(
		rename = "admin_delete_enabled",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub admin_delete_enabled: Option<Option<bool>>,
	#[serde(
		rename = "domains_enabled",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub domains_enabled: Option<Option<bool>>,
	/// Specify which enrollment modes to enable for your Organization Domains. Supported modes are 'automatic_invitation' & 'automatic_suggestion'.
	#[serde(rename = "domains_enrollment_modes", skip_serializing_if = "Option::is_none")]
	pub domains_enrollment_modes: Option<Vec<String>>,
	/// Specify what the default organization role is for an organization creator.
	#[serde(rename = "creator_role_id", skip_serializing_if = "Option::is_none")]
	pub creator_role_id: Option<String>,
	/// Specify what the default organization role is for the organization domains.
	#[serde(rename = "domains_default_role_id", skip_serializing_if = "Option::is_none")]
	pub domains_default_role_id: Option<String>,
}

impl UpdateInstanceOrganizationSettingsRequest {
	pub fn new() -> UpdateInstanceOrganizationSettingsRequest {
		UpdateInstanceOrganizationSettingsRequest {
			enabled: None,
			max_allowed_memberships: None,
			admin_delete_enabled: None,
			domains_enabled: None,
			domains_enrollment_modes: None,
			creator_role_id: None,
			domains_default_role_id: None,
		}
	}
}
