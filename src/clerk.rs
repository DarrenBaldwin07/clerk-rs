use crate::apis::configuration::Configuration;

/// Clerk Backend API version targeted by this crate.
pub const CLERK_API_VERSION: &str = "2026-05-12";

/// User agent sent with Clerk Backend API requests.
pub const USER_AGENT: &str = concat!("Clerk/v1 RustBindings/", env!("CARGO_PKG_VERSION"));

/// A lightweight owner for the configuration shared by generated API calls and
/// authentication middleware.
#[derive(Clone, Debug)]
pub struct Clerk {
	pub config: Configuration,
}

impl Clerk {
	pub fn new(configuration: Configuration) -> Self {
		Self { config: configuration }
	}

	pub fn from_secret_key(secret_key: impl Into<String>) -> Self {
		Self::new(Configuration::new(secret_key))
	}
}
