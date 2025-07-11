use crate::{
    clerk::Clerk,
    endpoints::ClerkGetEndpoint,
};
use serde_json::Value;

/// Helper function to check system connectivity and functionality
/// Returns the user list if successful
pub async fn check_system_functionality(client: &Clerk) -> Result<Value, reqwest::Error> {
    // Make a request to get user list as a basic connectivity test
    let response = client.get(ClerkGetEndpoint::GetUserList).await?;
    
    // Return the response
    Ok(response)
}

/// Helper function to get system version information
pub fn get_system_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}