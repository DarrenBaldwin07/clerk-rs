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
pub struct PhoneNumber {
	#[serde(rename = "id", skip_serializing_if = "Option::is_none")]
	pub id: Option<String>,
	/// String representing the object's type. Objects of the same type share the same value.
	#[serde(rename = "object")]
	pub object: Object,
	#[serde(rename = "phone_number")]
	pub phone_number: String,
	#[serde(rename = "reserved_for_second_factor", skip_serializing_if = "Option::is_none")]
	pub reserved_for_second_factor: Option<bool>,
	#[serde(rename = "default_second_factor", skip_serializing_if = "Option::is_none")]
	pub default_second_factor: Option<bool>,
	#[serde(rename = "reserved")]
	pub reserved: bool,
	#[serde(rename = "verification", deserialize_with = "Option::deserialize")]
	pub verification: Option<Box<crate::models::EmailAddressVerification>>,
	#[serde(rename = "linked_to")]
	pub linked_to: Vec<crate::models::IdentificationLink>,
	#[serde(
		rename = "backup_codes",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub backup_codes: Option<Option<Vec<String>>>,
}

impl PhoneNumber {
	pub fn new(
		object: Object,
		phone_number: String,
		reserved: bool,
		verification: Option<crate::models::EmailAddressVerification>,
		linked_to: Vec<crate::models::IdentificationLink>,
	) -> PhoneNumber {
		PhoneNumber {
			id: None,
			object,
			phone_number,
			reserved_for_second_factor: None,
			default_second_factor: None,
			reserved,
			verification: if let Some(x) = verification { Some(Box::new(x)) } else { None },
			linked_to,
			backup_codes: None,
		}
	}
}

/// String representing the object's type. Objects of the same type share the same value.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Object {
	#[serde(rename = "phone_number")]
	PhoneNumber,
}

impl Default for Object {
	fn default() -> Object {
		Self::PhoneNumber
	}
}
