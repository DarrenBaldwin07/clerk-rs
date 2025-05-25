use std::collections::HashMap;
use serde::Serialize;

/// Error type for input validation
#[derive(Debug, Clone)]
pub enum ValidationError {
    /// Parameter is missing or empty
    EmptyOrNull(String),
    /// Parameter doesn't match expected format
    InvalidFormat(String),
    /// Multiple validation errors
    Multiple(Vec<ValidationError>),
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::EmptyOrNull(param) => write!(f, "Parameter '{}' cannot be empty or null", param),
            ValidationError::InvalidFormat(msg) => write!(f, "{}", msg),
            ValidationError::Multiple(errors) => {
                for (i, error) in errors.iter().enumerate() {
                    if i > 0 {
                        write!(f, "; ")?;
                    }
                    write!(f, "{}", error)?;
                }
                Ok(())
            }
        }
    }
}

impl std::error::Error for ValidationError {}

/// Trait for validating input parameters
pub trait Validate {
    /// Validates the input and returns a ValidationError if invalid
    fn validate(&self) -> Result<(), ValidationError>;
}

/// Validates that a string parameter is not empty
pub fn validate_str_param(param_name: &str, param_value: &str) -> Result<(), ValidationError> {
    if param_value.trim().is_empty() {
        return Err(ValidationError::EmptyOrNull(param_name.to_string()));
    }
    Ok(())
}

/// Validates that a string parameter is not empty and satisfies optional additional checks
pub fn validate_str_param_with(
    param_name: &str, 
    param_value: &str,
    additional_check: impl FnOnce(&str) -> Result<(), String>
) -> Result<(), ValidationError> {
    validate_str_param(param_name, param_value)?;
    
    additional_check(param_value).map_err(|msg| {
        ValidationError::InvalidFormat(format!("Parameter '{}': {}", param_name, msg))
    })
}

/// Validates that an Option<String> parameter is Some and not empty if provided
pub fn validate_optional_str_param(param_name: &str, param_value: &Option<String>) -> Result<(), ValidationError> {
    if let Some(value) = param_value {
        validate_str_param(param_name, value)?;
    }
    Ok(())
}

/// Validates multiple parameters at once
pub fn validate_params(validations: Vec<Result<(), ValidationError>>) -> Result<(), ValidationError> {
    let errors: Vec<ValidationError> = validations
        .into_iter()
        .filter_map(|result| result.err())
        .collect();
    
    if errors.is_empty() {
        Ok(())
    } else if errors.len() == 1 {
        Err(errors.into_iter().next().unwrap())
    } else {
        Err(ValidationError::Multiple(errors))
    }
}

/// Validates a request object before sending it to the API
pub fn validate_request<T: Serialize>(request: &Option<T>) -> Result<(), ValidationError> {
    if request.is_none() {
        return Err(ValidationError::EmptyOrNull("request".to_string()));
    }
    Ok(())
}

/// Validates that a collection is not empty
pub fn validate_collection<T>(param_name: &str, collection: &[T]) -> Result<(), ValidationError> {
    if collection.is_empty() {
        return Err(ValidationError::EmptyOrNull(format!("{} collection", param_name)));
    }
    Ok(())
}

/// Validates a map to ensure it's not empty
pub fn validate_map<K, V>(param_name: &str, map: &HashMap<K, V>) -> Result<(), ValidationError> {
    if map.is_empty() {
        return Err(ValidationError::EmptyOrNull(format!("{} map", param_name)));
    }
    Ok(())
}