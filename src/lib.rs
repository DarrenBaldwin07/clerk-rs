#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;
extern crate dotenv;

use std::env;

pub mod apis;
pub mod clerk;
pub mod endpoints;
pub mod models;
pub(crate) mod util;
pub mod validators;
pub use apis::configuration::ClerkConfiguration;
pub use models as ClerkModels;

/// Helper function to load Clerk secret key from environment variables
/// 
/// This function will:
/// 1. Try to load .env file if available (does not error if file is missing)
/// 2. Get the CLERK_SECRET_KEY from environment variables
/// 3. Return the secret key as a String
///
/// # Example
/// 
/// ```rust
/// use clerk_rs::load_clerk_secret_key;
/// 
/// let secret_key = load_clerk_secret_key();
/// let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
/// let clerk = Clerk::new(config);
/// ```
pub fn load_clerk_secret_key() -> String {
    // Try to load from .env file (doesn't error if file not found)
    dotenv::dotenv().ok();
    
    // Get secret key from environment variable
    env::var("CLERK_SECRET_KEY")
        .expect("CLERK_SECRET_KEY environment variable must be set")
}
