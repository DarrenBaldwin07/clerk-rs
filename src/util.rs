use regex::Regex;
use std::error::Error;
use std::fmt;

/// Method for converting a dynamic query string (ex: /getUser/{id}) and converting it to something like: "/getUser/1741897489174891"
pub fn generate_path_from_params(route_path: String, params: Vec<&str>) -> String {
	let dynamic_regex = Regex::new(r"\{[^\{\}]*\}").unwrap();
	let mut matches: Vec<String> = Vec::new();
	let mut new_route_path = route_path;

	// Get every matched segment
	for capture in dynamic_regex.captures_iter(&new_route_path) {
		let dynamic_segement = capture[0].to_string();
		matches.push(dynamic_segement);
	}

	// Now that we have every match lets replace every string in the route path with user specified route params
	for (index, str_match) in matches.iter().enumerate() {
		let parsed_path = new_route_path.replacen(str_match, &params[index], 1);
		new_route_path = parsed_path;
	}

	new_route_path
}

/// Error type for token validation failures
#[derive(Debug)]
pub enum TokenValidationError {
    EmptyToken,
    InvalidFormat(String),
}

impl fmt::Display for TokenValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TokenValidationError::EmptyToken => write!(f, "Token ID cannot be empty"),
            TokenValidationError::InvalidFormat(msg) => write!(f, "Invalid token format: {}", msg),
        }
    }
}

impl Error for TokenValidationError {}

/// Validates a token ID for proper format
/// 
/// This function ensures that token IDs follow the proper format.
/// Currently checks:
/// - Token is not empty
/// - Token follows the expected pattern (alphanumeric characters and underscores)
pub fn validate_token_id(token_id: &str) -> Result<(), TokenValidationError> {
    if token_id.is_empty() {
        return Err(TokenValidationError::EmptyToken);
    }
    
    // Check for proper format - Clerk token IDs typically follow a specific pattern
    // This regex pattern should be updated to match the exact format Clerk uses for token IDs
    let token_regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();
    if !token_regex.is_match(token_id) {
        return Err(TokenValidationError::InvalidFormat(
            "Token ID must contain only alphanumeric characters and underscores".to_string()
        ));
    }
    
    Ok(())
}
