use clerk_rs::{clerk::Clerk, endpoints::ClerkGetEndpoint, ClerkConfiguration};
use dotenv::dotenv;
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Load environment variables from .env file
	dotenv().ok();
	
	// Get the secret key from environment variable or provide an error message
	let secret_key = env::var("CLERK_SECRET_KEY")
		.expect("CLERK_SECRET_KEY environment variable is not set");
		
	// Create a new clerk configuration so that we can make authed requests
	let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
	// Initialize our Clerk client with the newly created configuration
	let client = Clerk::new(config);
	// Make the api call to the specified clerk endpoint
	let _res = client.get(ClerkGetEndpoint::GetUserList).await?;

	// ...
	Ok(())
}
