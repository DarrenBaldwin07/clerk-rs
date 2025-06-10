use clerk_rs::{clerk::Clerk, endpoints::ClerkGetEndpoint, get_clerk_secret_key, ClerkConfiguration};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Get the clerk secret key from the CLERK_SECRET_KEY environment variable
	let secret_key = get_clerk_secret_key();
	
	// Create a new clerk configuration so that we can make authed requests
	let config = ClerkConfiguration::new(None, None, secret_key, None);
	// Initialize our Clerk client with the newly created configuration
	let client = Clerk::new(config);
	// Make the api call to the specified clerk endpoint
	let _res = client.get(ClerkGetEndpoint::GetUserList).await?;

	// ...
	Ok(())
}
