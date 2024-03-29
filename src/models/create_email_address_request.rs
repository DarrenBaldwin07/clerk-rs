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
pub struct CreateEmailAddressRequest {
	/// The ID representing the user
	#[serde(rename = "user_id", skip_serializing_if = "Option::is_none")]
	pub user_id: Option<String>,
	/// The new email address. Must adhere to the RFC 5322 specification for email address format.
	#[serde(rename = "email_address", skip_serializing_if = "Option::is_none")]
	pub email_address: Option<String>,
	/// When created, the email address will be marked as verified.
	#[serde(
		rename = "verified",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub verified: Option<Option<bool>>,
	/// Create this email address as the primary email address for the user. Default: false, unless it is the first email address.
	#[serde(
		rename = "primary",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub primary: Option<Option<bool>>,
}

impl CreateEmailAddressRequest {
	pub fn new() -> CreateEmailAddressRequest {
		CreateEmailAddressRequest {
			user_id: None,
			email_address: None,
			verified: None,
			primary: None,
		}
	}
}
