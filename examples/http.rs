use clerk_rs::{clerk::Clerk, endpoints::ClerkGetEndpoint, ClerkConfiguration};
use tokio;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// For this example, ensure CLERK_SECRET_KEY is set in your environment
	// If not set, you can uncomment the line below to set it for testing purposes only
	// env::set_var("CLERK_SECRET_KEY", "sk_test_key"); // NOTE: In production, use proper env management
	
	// Create a new clerk configuration so that we can make authed requests
	// Secret key will be automatically loaded from the CLERK_SECRET_KEY environment variable
	let config = ClerkConfiguration::new(None, None, None, None);
	// Initialize our Clerk client with the newly created configuration
	let client = Clerk::new(config);
	// Make the api call to the specified clerk endpoint
	let _res = client.get(ClerkGetEndpoint::GetUserList).await?;

	// ...
	Ok(())
}
