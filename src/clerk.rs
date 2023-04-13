use crate::apis::configuration;
use serde_json::{self, Value};


// Default user agent used for clerk-rs sdk (this is sent with every clerk api request)
pub const USER_AGENT: &str = concat!("Clerk/v1 RustBindings/", env!("CARGO_PKG_VERSION"));

pub struct Clerk {
    pub config: configuration::ClerkConfiguration
}

impl Clerk {
    // Create a new Clerk SDK client for making requests out to the public Clerk api:
    pub fn new(clerk_configuration: configuration::ClerkConfiguration) -> Self {
        Self {
            config: clerk_configuration
        }
    }

    /// Function for submitting  a GET request to a specifed clerk api endpoint
    pub async fn get(&self, endpoint: String) -> Result<serde_json::value::Value, reqwest::Error> {
        let url = format!("{}{}", self.config.base_path, endpoint);

        match self.config.client.get(&url).send().await {
            Ok(response) => {
                match response.json::<Value>().await {
                    Ok(user) => Ok(user),
                    Err(e) => Err(e),
                }
            }
            Err(e) => Err(e)
        }
    }
}
