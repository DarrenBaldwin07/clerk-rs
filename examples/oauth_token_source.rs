use clerk_rs::{
    apis::{
        configuration::{ClerkConfiguration, ReuseTokenSource, StaticTokenSource, Token, TokenSource},
        emails_api::Email,
    },
    clerk::Clerk,
    models::CreateEmailRequest,
};
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio;

// Example custom token source implementation that fetches tokens from an external service
struct CustomTokenSource {
    // Fields like client_id, client_secret, etc. would go here
}

#[async_trait::async_trait]
impl TokenSource for CustomTokenSource {
    async fn token(&self) -> Result<Token, String> {
        // In a real implementation, this would fetch a new token from an OAuth2 provider
        // For this example, we'll just return a mock token
        let expiry = SystemTime::now()
            .checked_add(Duration::from_secs(3600))
            .ok_or_else(|| "Failed to calculate expiry time".to_string())?;

        Ok(Token::new(
            "mock_access_token".to_string(),
            "Bearer".to_string(),
            Some("mock_refresh_token".to_string()),
            Some(expiry),
        ))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example 1: Using a static token source
    println!("Example 1: Using a static token source");
    let static_token_source = Arc::new(StaticTokenSource::new("static_access_token".to_string()));
    
    let config = ClerkConfiguration::new(
        None,
        None,
        None,
        None,
        Some(static_token_source.clone()),
    );
    let client = Clerk::new(config);
    
    // The API call will use the token from the token source
    let new_email = CreateEmailRequest::new();
    let result = Email::create(&client, Some(new_email)).await;
    println!("API result: {:?}", result);

    // Example 2: Using a reusable token source with caching
    println!("\nExample 2: Using a reusable token source with caching");
    
    // Create a custom token source
    let custom_source = Arc::new(CustomTokenSource {});
    
    // Wrap it in a reusable token source that will cache tokens until they expire
    let reusable_token_source = Arc::new(ReuseTokenSource::new(custom_source, None));
    
    let config = ClerkConfiguration::new(
        None,
        None,
        None,
        None,
        Some(reusable_token_source),
    );
    let client = Clerk::new(config);
    
    // The API call will use the token from the token source
    let new_email = CreateEmailRequest::new();
    let result = Email::create(&client, Some(new_email)).await;
    println!("API result: {:?}", result);

    Ok(())
}