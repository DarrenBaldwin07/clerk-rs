use clerk_rs::{apis::emails_api::Email, clerk::Clerk, get_clerk_secret_key, models::CreateEmailRequest, ClerkConfiguration};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Get the clerk secret key from the CLERK_SECRET_KEY environment variable
	let secret_key = get_clerk_secret_key();
	
	// Create a new clerk configuration so that we can make authed requests
	let config = ClerkConfiguration::new(None, None, secret_key, None);
	// Initialize our Clerk client with the newly created configuration
	let client = Clerk::new(config);
	// Create a new email using the Email api...
	let new_email = CreateEmailRequest::new();

	let _ = Email::create(&client, Some(new_email)).await;

	Ok(())
}
