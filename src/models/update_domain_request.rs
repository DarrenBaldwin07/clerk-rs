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
pub struct UpdateDomainRequest {
	/// The new domain name. For development instances, can contain the port, i.e `myhostname:3000`. For production instances, must be a valid FQDN, i.e `mysite.com`. Cannot contain protocol scheme.
	#[serde(
		rename = "name",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub name: Option<Option<String>>,
	/// The full URL of the proxy that will forward requests to Clerk's Frontend API. Can only be updated for production instances.
	#[serde(
		rename = "proxy_url",
		default,
		with = "::serde_with::rust::double_option",
		skip_serializing_if = "Option::is_none"
	)]
	pub proxy_url: Option<Option<String>>,
}

impl UpdateDomainRequest {
	pub fn new() -> UpdateDomainRequest {
		UpdateDomainRequest { name: None, proxy_url: None }
	}
}
