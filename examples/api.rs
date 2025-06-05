use clerk_rs::{apis::emails_api::Email, clerk::Clerk, models::CreateEmailRequest, ClerkConfiguration};
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
	// Create a new email using the Email api...
	let new_email = CreateEmailRequest::new();

	let _ = Email::create(&client, Some(new_email)).await;

	Ok(())
}
