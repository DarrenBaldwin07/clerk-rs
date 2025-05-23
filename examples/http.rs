use clerk_rs::{clerk::Clerk, endpoints::ClerkGetEndpoint, ClerkConfiguration};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Create a new clerk configuration using the CLERK_SECRET_KEY environment variable
	let config = ClerkConfiguration::from_env()?;
	// Initialize our Clerk client with the newly created configuration
	let client = Clerk::new(config);
	// Make the api call to the specified clerk endpoint
	let _res = client.get(ClerkGetEndpoint::GetUserList).await?;

	// ...
	Ok(())
}
