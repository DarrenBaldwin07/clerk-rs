use clerk_rs::{apis::emails_api::Email, clerk::Clerk, models::CreateEmailRequest, ClerkConfiguration};
use std::env;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Get secret key from environment variable
	let secret_key = env::var("CLERK_SECRET_KEY").unwrap_or_else(|_| {
		eprintln!("Warning: CLERK_SECRET_KEY environment variable not set");
		String::new()
	});
	
	// Create a new clerk configuration so that we can make authed requests
	let config = ClerkConfiguration::new(None, None, if !secret_key.is_empty() { Some(secret_key) } else { None }, None);
	// Initialize our Clerk client with the newly created configuration
	let client = Clerk::new(config);
	// Create a new email using the Email api...
	let new_email = CreateEmailRequest::new();

	let _ = Email::create(&client, Some(new_email)).await;

	Ok(())
}
