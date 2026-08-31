use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct ResponseContent<T> {
	pub status: reqwest::StatusCode,
	pub content: String,
	pub entity: Option<T>,
}

#[derive(Debug)]
pub enum Error<T> {
	Reqwest(reqwest::Error),
	Serde(serde_json::Error),
	Io(std::io::Error),
	ResponseError(ResponseContent<T>),
}

impl<T> fmt::Display for Error<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let (module, e) = match self {
			Error::Reqwest(e) => ("reqwest", e.to_string()),
			Error::Serde(e) => ("serde", e.to_string()),
			Error::Io(e) => ("IO", e.to_string()),
			Error::ResponseError(e) => ("response", format!("status code {}", e.status)),
		};
		write!(f, "error in {}: {}", module, e)
	}
}

impl<T: fmt::Debug> error::Error for Error<T> {
	fn source(&self) -> Option<&(dyn error::Error + 'static)> {
		Some(match self {
			Error::Reqwest(e) => e,
			Error::Serde(e) => e,
			Error::Io(e) => e,
			Error::ResponseError(_) => return None,
		})
	}
}

impl<T> From<reqwest::Error> for Error<T> {
	fn from(e: reqwest::Error) -> Self {
		Error::Reqwest(e)
	}
}

impl<T> From<serde_json::Error> for Error<T> {
	fn from(e: serde_json::Error) -> Self {
		Error::Serde(e)
	}
}

impl<T> From<std::io::Error> for Error<T> {
	fn from(e: std::io::Error) -> Self {
		Error::Io(e)
	}
}

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

/// Internal use only
/// A content type supported by this client.
#[allow(dead_code)]
enum ContentType {
	Json,
	Text,
	Unsupported(String),
}

impl From<&str> for ContentType {
	fn from(content_type: &str) -> Self {
		if content_type.starts_with("application") && content_type.contains("json") {
			return Self::Json;
		} else if content_type.starts_with("text/plain") {
			return Self::Text;
		} else {
			return Self::Unsupported(content_type.to_string());
		}
	}
}

pub mod actor_tokens_api;
pub mod admin_portal_link_tokens_api;
pub mod agent_tasks_api;
pub mod allow_list_block_list_api;
pub mod api_keys_api;
pub mod beta_features_api;
pub mod billing_api;
pub mod clients_api;
pub mod directories_api;
pub mod domains_api;
pub mod email_addresses_api;
pub mod email_sms_templates_api;
pub mod enterprise_connections_api;
pub mod instance_settings_api;
pub mod invitations_api;
pub mod jwks_api;
pub mod jwt_templates_api;
pub mod m2_m_tokens_api;
pub mod machines_api;
pub mod miscellaneous_api;
pub mod o_auth_access_tokens_api;
pub mod o_auth_applications_api;
pub mod organization_domains_api;
pub mod organization_invitations_api;
pub mod organization_memberships_api;
pub mod organization_permissions_api;
pub mod organization_roles_api;
pub mod organizations_api;
pub mod phone_numbers_api;
pub mod proxy_checks_api;
pub mod redirect_urls_api;
pub mod role_sets_api;
pub mod saml_connections_api;
pub mod scim_directories_api;
pub mod sessions_api;
pub mod sign_in_tokens_api;
pub mod sign_ups_api;
pub mod testing_tokens_api;
pub mod users_api;
pub mod waitlist_entries_api;
pub mod webhooks_api;

pub mod configuration;
