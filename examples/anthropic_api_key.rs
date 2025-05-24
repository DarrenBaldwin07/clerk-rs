use clerk_rs::{
    apis::{
        configuration::ApiKey,
        organizations_api::Organization,
    },
    clerk::Clerk,
    models::UpdateOrganizationRequest,
    ClerkConfiguration,
};

/// This example shows how to set and use a custom Anthropic API key for an organization
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a clerk client with your secret key
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk_client = Clerk::new(config);

    // Organization ID to update
    let organization_id = "org_123456789";

    // 1. Update the organization with a custom Anthropic API key
    let mut update_request = UpdateOrganizationRequest::new();
    update_request.anthropic_api_key = Some(Some("sk-ant-api123456789".to_string()));

    // Send the update request
    let updated_org = Organization::update_organization(
        &clerk_client,
        organization_id,
        update_request
    ).await?;

    println!("Updated organization: {:?}", updated_org);

    // 2. Fetch the organization to verify the API key was set
    let org = Organization::get_organization(
        &clerk_client,
        organization_id
    ).await?;

    // In a real application, you would now use the API key for Anthropic requests
    // For example, you could create a new configuration for Anthropic with:
    // 
    // if let Some(anthropic_api_key) = org.settings.anthropic_api_key {
    //     let anthropic_config = AnthropicConfiguration::new(
    //         ApiKey {
    //             prefix: Some("Bearer".to_string()),
    //             key: anthropic_api_key,
    //         }
    //     );
    //     // Use the anthropic_config for API calls...
    // } else {
    //     // Fall back to environment variable API key
    //     let api_key = std::env::var("ANTHROPIC_API_KEY")?;
    //     let anthropic_config = AnthropicConfiguration::new(
    //         ApiKey {
    //             prefix: Some("Bearer".to_string()),
    //             key: api_key,
    //         }
    //     );
    //     // Use the anthropic_config for API calls...
    // }

    println!("Successfully demonstrated API key management for organization: {}", organization_id);

    Ok(())
}