use clerk_rs::{clerk::Clerk, endpoints::ClerkGetEndpoint, ClerkConfiguration};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Get the secret key from environment variable
	let secret_key = env::var("CLERK_SECRET_KEY").expect("CLERK_SECRET_KEY environment variable not set");
	
	// Create a new clerk configuration so that we can make authed requests
	let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
	// Initialize our Clerk client with the newly created configuration
	let client = Clerk::new(config);
	// Make the api call to the specified clerk endpoint
	let _res = client.get(ClerkGetEndpoint::GetUserList).await?;

	// ...
	Ok(())
}
