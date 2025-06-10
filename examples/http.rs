use clerk_rs::{clerk::Clerk, endpoints::ClerkGetEndpoint, ClerkConfiguration};
use dotenv::dotenv;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Load environment variables from .env file if it exists
	dotenv().ok();

	// Create a new clerk configuration from environment variable
	let config = ClerkConfiguration::from_env().expect("CLERK_SECRET_KEY environment variable not set");
	// Initialize our Clerk client with the newly created configuration
	let client = Clerk::new(config);
	// Make the api call to the specified clerk endpoint
	let _res = client.get(ClerkGetEndpoint::GetUserList).await?;

	// ...
	Ok(())
}
