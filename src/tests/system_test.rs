use crate::{
    apis::configuration::ClerkConfiguration,
    clerk::Clerk,
    endpoints::ClerkGetEndpoint,
    util::check_system_functionality,
};
use serde_json::json;
use std::error::Error;
use mockito::{mock, server_url};

#[tokio::test]
async fn test_system_functionality() -> Result<(), Box<dyn Error>> {
    // Setup mock server
    let mock_server = mock("GET", "/v1/users")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(r#"{"data": [{"id": "user_123", "email": "test@example.com"}]}"#)
        .create();
    
    // Create configuration with mock server URL
    let mut config = ClerkConfiguration::new(
        None, 
        None, 
        Some("test_token".to_string()), 
        None
    );
    
    // Override base path to use mock server
    config.base_path = format!("{}/v1", server_url());
    
    // Initialize client with mock configuration
    let client = Clerk::new(config);
    
    // Make API call using our system functionality check
    let response = check_system_functionality(&client).await?;
    
    // Verify mock was called
    mock_server.assert();
    
    // Verify response data structure
    assert!(response.is_object());
    assert!(response.get("data").is_some());
    
    let data = response.get("data").unwrap().as_array().unwrap();
    assert_eq!(data.len(), 1);
    
    let user = &data[0];
    assert_eq!(user.get("id").unwrap().as_str().unwrap(), "user_123");
    assert_eq!(user.get("email").unwrap().as_str().unwrap(), "test@example.com");
    
    Ok(())
}

#[tokio::test]
async fn test_error_handling() -> Result<(), Box<dyn Error>> {
    // Setup mock server with error response
    let mock_server = mock("GET", "/v1/users")
        .with_status(401)
        .with_header("content-type", "application/json")
        .with_body(r#"{"error": {"message": "Unauthorized access"}}"#)
        .create();
    
    // Create configuration with mock server URL
    let mut config = ClerkConfiguration::new(
        None, 
        None, 
        Some("invalid_token".to_string()), 
        None
    );
    
    // Override base path to use mock server
    config.base_path = format!("{}/v1", server_url());
    
    // Initialize client with mock configuration
    let client = Clerk::new(config);
    
    // Make API call - this should still succeed at the HTTP level because we're
    // mocking the response, but the response contains an error object
    let response = client.get(ClerkGetEndpoint::GetUserList).await?;
    
    // Verify mock was called
    mock_server.assert();
    
    // Verify error response structure
    assert!(response.is_object());
    assert!(response.get("error").is_some());
    assert_eq!(
        response.get("error").unwrap().get("message").unwrap().as_str().unwrap(),
        "Unauthorized access"
    );
    
    Ok(())
}