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
pub struct CreateEmailRequest {
	/// The email name portion of the sending email address. <br/>e.g.: `from_email_name=info` will send from info@example.com
	#[serde(rename = "from_email_name", skip_serializing_if = "Option::is_none")]
	pub from_email_name: Option<String>,
	/// The subject of the email.
	#[serde(rename = "subject", skip_serializing_if = "Option::is_none")]
	pub subject: Option<String>,
	/// The body of the email.
	#[serde(rename = "body", skip_serializing_if = "Option::is_none")]
	pub body: Option<String>,
	/// The ID of the email address to send to.
	#[serde(
		rename = "email_address_id",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub email_address_id: Option<Option<String>>,
}

impl CreateEmailRequest {
	pub fn new() -> CreateEmailRequest {
		CreateEmailRequest {
			from_email_name: None,
			subject: None,
			body: None,
			email_address_id: None,
		}
	}
}
