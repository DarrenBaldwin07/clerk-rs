use clerk_rs::{clerk::Clerk, endpoints::ClerkGetEndpoint, ClerkConfiguration, load_clerk_secret_key};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Get secret key from environment variable using helper function
	let secret_key = load_clerk_secret_key();
	
	// Create a new clerk configuration so that we can make authed requests
	let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
	// Initialize our Clerk client with the newly created configuration
	let client = Clerk::new(config);
	// Make the api call to the specified clerk endpoint
	let _res = client.get(ClerkGetEndpoint::GetUserList).await?;

	// ...
	Ok(())
}
