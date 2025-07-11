use clerk_rs::{
    clerk::Clerk,
    ClerkConfiguration,
    util::{check_system_functionality, get_system_version},
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get and print system version
    println!("Clerk-rs version: {}", get_system_version());
    
    // Create a new clerk configuration with your API key
    let config = ClerkConfiguration::new(
        None,
        None,
        Some("your_secret_key".to_string()), // Replace with your actual API key
        None
    );
    
    // Initialize our Clerk client with the configuration
    let client = Clerk::new(config);
    
    // Check system functionality
    println!("Checking system functionality...");
    match check_system_functionality(&client).await {
        Ok(response) => {
            println!("System check successful!");
            println!("Response: {}", response);
        },
        Err(e) => {
            println!("System check failed: {}", e);
        }
    }
    
    Ok(())
}