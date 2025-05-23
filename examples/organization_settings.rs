use clerk_rs::{
    apis::instance_settings_api::InstanceSettings,
    clerk::Clerk,
    models::{ModelType, UpdateInstanceOrganizationSettingsRequest},
    ClerkConfiguration,
};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new clerk configuration so that we can make authed requests
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    
    // Initialize our Clerk client with the newly created configuration
    let client = Clerk::new(config);
    
    // Create a request to update the organization settings
    let mut request = UpdateInstanceOrganizationSettingsRequest::new();
    
    // Set the selected model to GPT-4
    request = request.with_selected_model(ModelType::Gpt4.as_str());
    
    // Or set the selected model to Claude 3 Opus
    // request = request.with_selected_model(ModelType::Claude3Opus.as_str());
    
    // Make sure all available models are set
    request = request.with_default_available_models();
    
    // Or you can specify specific models to be available
    // let custom_models = vec![
    //     ModelType::Gpt35Turbo.as_str().to_string(),
    //     ModelType::Gpt4.as_str().to_string(),
    //     ModelType::Claude3Opus.as_str().to_string(),
    // ];
    // request = request.with_available_models(custom_models);
    
    // Update the organization settings
    let updated_settings = InstanceSettings::update_instance_organization_settings(&client, Some(request)).await?;
    
    // Print the selected model
    println!("Selected model: {}", updated_settings.get_selected_model());
    
    // Print all available models
    println!("Available models:");
    for model in updated_settings.get_available_models() {
        println!("  - {}", model);
    }

    Ok(())
}