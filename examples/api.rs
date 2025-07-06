use clerk_rs::{apis::emails_api::Email, clerk::Clerk, models::CreateEmailRequest, ClerkConfiguration};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	// Create a new clerk configuration so that we can make authed requests
	let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
	// Initialize our Clerk client with the newly created configuration
	let client = Clerk::new(config);
	// Create a new email using the Email api...
	let mut new_email = CreateEmailRequest::new();
	
	// Set required fields to avoid undefined variables
	new_email.from_email_name = Some("support".to_string());
	new_email.subject = Some("Test Email Subject".to_string());
	new_email.body = Some("<h1>Hello World</h1><p>This is a test email from Clerk API</p>".to_string());
	// Email address ID is optional but can be specified if needed
	// new_email.email_address_id = Some(Some("email_address_id".to_string()));

	let _ = Email::create(&client, Some(new_email)).await;

	Ok(())
}
