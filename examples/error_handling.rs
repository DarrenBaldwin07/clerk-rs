use clerk_rs::{clerk::Clerk, error::Error};

#[tokio::main]
async fn main() {
    // Initialize a Clerk client with a non-existent API token to test error handling
    let clerk = Clerk::new("sk_test_invalid_token").unwrap();
    
    // Attempt to create a sign-in token (this will fail due to invalid token)
    let result = clerk_rs::apis::sign_in_tokens_api::SignInToken::create_sign_in_token(
        &clerk,
        Some(clerk_rs::models::CreateSignInTokenRequest {
            user_id: "user_123".to_string(),
            expires_in_seconds: Some(3600),
        }),
    )
    .await;
    
    // Handle the error appropriately
    match result {
        Ok(_) => {
            println!("This should not happen with an invalid token");
        }
        Err(Error::ApiError(api_error)) => {
            println!("API Error: {}", api_error);
            
            // Demonstrate the improved error handling capabilities
            if let Some(message) = api_error.message() {
                println!("Error message: {}", message);
            }
            
            if let Some(code) = api_error.code() {
                println!("Error code: {}", code);
            }
            
            println!("Status code: {}", api_error.status);
        }
        Err(e) => {
            println!("Other error: {}", e);
        }
    }
}