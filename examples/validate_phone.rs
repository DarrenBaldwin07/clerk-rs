use clerk_rs::models::CreatePhoneNumberRequest;

fn main() {
    // Example of valid phone number
    let mut request = CreatePhoneNumberRequest::new();
    request.phone_number = Some("+12025550123".to_string());
    request.user_id = Some("user_123".to_string());
    
    match request.validate() {
        Ok(_) => println!("Valid phone number request"),
        Err(e) => println!("Error: {}", e),
    }
    
    // Example of invalid phone number
    let mut request = CreatePhoneNumberRequest::new();
    request.phone_number = Some("12025550123".to_string()); // Missing + prefix
    request.user_id = Some("user_123".to_string());
    
    match request.validate() {
        Ok(_) => println!("Valid phone number request"),
        Err(e) => println!("Error: {}", e),
    }
    
    // Example of empty user_id
    let mut request = CreatePhoneNumberRequest::new();
    request.phone_number = Some("+12025550123".to_string());
    request.user_id = Some("".to_string());
    
    match request.validate() {
        Ok(_) => println!("Valid phone number request"),
        Err(e) => println!("Error: {}", e),
    }
}