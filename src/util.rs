use regex::Regex;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Parameter cannot be empty: {0}")]
    EmptyParameter(String),
    
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),
    
    #[error("Invalid parameter count: expected {0}, got {1}")]
    ParameterCountMismatch(usize, usize),
    
    #[error("Request body is required")]
    MissingRequestBody,
}

/// Validates that a string parameter is not empty
pub fn validate_string_param(param: &str, param_name: &str) -> Result<(), ValidationError> {
    if param.trim().is_empty() {
        return Err(ValidationError::EmptyParameter(param_name.to_string()));
    }
    Ok(())
}

/// Validates that an optional request body is present
pub fn validate_request_body<T>(body: &Option<T>, body_name: &str) -> Result<(), ValidationError> {
    if body.is_none() {
        return Err(ValidationError::MissingParameter(body_name.to_string()));
    }
    Ok(())
}

/// Method for converting a dynamic query string (ex: /getUser/{id}) and converting it to something like: "/getUser/1741897489174891"
pub fn generate_path_from_params(route_path: String, params: Vec<&str>) -> Result<String, ValidationError> {
    let dynamic_regex = Regex::new(r"\{[^\{\}]*\}").unwrap();
    let mut matches: Vec<String> = Vec::new();
    let mut new_route_path = route_path;

    // Get every matched segment
    for capture in dynamic_regex.captures_iter(&new_route_path) {
        let dynamic_segment = capture[0].to_string();
        matches.push(dynamic_segment);
    }

    // Validate that we have enough parameters
    if matches.len() != params.len() {
        return Err(ValidationError::ParameterCountMismatch(matches.len(), params.len()));
    }

    // Validate that all parameters are non-empty
    for (i, param) in params.iter().enumerate() {
        if param.trim().is_empty() {
            return Err(ValidationError::EmptyParameter(format!("Parameter at index {}", i)));
        }
    }

    // Now that we have every match lets replace every string in the route path with user specified route params
    for (index, str_match) in matches.iter().enumerate() {
        let parsed_path = new_route_path.replacen(str_match, &params[index], 1);
        new_route_path = parsed_path;
    }

    Ok(new_route_path)
}
