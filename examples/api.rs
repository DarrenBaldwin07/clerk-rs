use clerk_rs::{apis::emails_api::Email, clerk::Clerk, models::CreateEmailRequest, ClerkConfiguration};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Create a new clerk configuration so that we can make authed requests
	let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
	// Initialize our Clerk client with the newly created configuration
	let client = Clerk::new(config);
	// Create a new email using the Email api...
	let new_email = CreateEmailRequest::new();

	let _ = Email::create(&client, Some(new_email)).await;

	Ok(())
}
