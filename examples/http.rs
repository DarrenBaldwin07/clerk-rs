use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new clerk configuration so that we can make authed requests
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    // Initialize our Clerk client with the newly created configuration
    let client = Clerk::new(config);
    // Make the api call to the specified clerk endpoint
    let res = client.get(ClerkGetEndpoint::GetUserList).await?;

    Ok(())
}
