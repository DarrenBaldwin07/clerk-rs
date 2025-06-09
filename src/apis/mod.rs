// Re-export error types from crate::error
pub use crate::error::{ApiError, Error};

pub fn urlencode<T: AsRef<str>>(s: T) -> String {
	::url::form_urlencoded::byte_serialize(s.as_ref().as_bytes()).collect()
}

pub fn parse_deep_object(prefix: &str, value: &serde_json::Value) -> Vec<(String, String)> {
	if let serde_json::Value::Object(object) = value {
		let mut params = vec![];

		for (key, value) in object {
			match value {
				serde_json::Value::Object(_) => params.append(&mut parse_deep_object(&format!("{}[{}]", prefix, key), value)),
				serde_json::Value::Array(array) => {
					for (i, value) in array.iter().enumerate() {
						params.append(&mut parse_deep_object(&format!("{}[{}][{}]", prefix, key, i), value));
					}
				}
				serde_json::Value::String(s) => params.push((format!("{}[{}]", prefix, key), s.clone())),
				_ => params.push((format!("{}[{}]", prefix, key), value.to_string())),
			}
		}

		return params;
	}

	unimplemented!("Only objects are supported with style=deepObject")
}

pub mod actor_tokens_api;
pub mod allow_list_block_list_api;
pub mod beta_features_api;
pub mod clients_api;
pub mod configuration;
pub mod email_addresses_api;
pub mod email_sms_templates_api;
pub mod emails_api;
pub mod instance_settings_api;
pub mod invitations_api;
pub mod jwks_api;
pub mod jwt_templates_api;
pub mod miscellaneous_api;
pub mod organization_invitations_api;
pub mod organization_memberships_api;
pub mod organizations_api;
pub mod phone_numbers_api;
pub mod redirect_urls_api;
pub mod sessions_api;
pub mod sign_in_tokens_api;
pub mod sign_ups_api;
pub mod users_api;
pub mod users_middleware;
pub mod webhooks_api;
