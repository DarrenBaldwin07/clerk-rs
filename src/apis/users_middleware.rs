/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 */

use crate::models::{CreateUserRequest, UpdateUserRequest};
use crate::validators::password::PasswordValidator;

/// Middleware for validating user creation parameters
pub struct UsersMiddleware;

impl UsersMiddleware {
    /// Minimum password length requirement
    const MIN_PASSWORD_LENGTH: usize = 8;
    
    /// Extract context information from a create user request to determine if password checks can be skipped
    fn extract_context_from_request(request: &CreateUserRequest) -> String {
        // Check if this appears to be a migration scenario
        // Indicators: presence of external_id, password_digest, or specific metadata
        let mut context_indicators = Vec::new();
        
        if request.external_id.is_some() {
            context_indicators.push("external_id");
        }
        
        if request.password_digest.is_some() {
            context_indicators.push("password_digest");
        }
        
        if let Some(metadata) = &request.private_metadata {
            if metadata.to_string().contains("migration") || metadata.to_string().contains("import") {
                context_indicators.push("migration_metadata");
            }
        }
        
        if context_indicators.is_empty() {
            "regular_creation".to_string()
        } else {
            format!("migration:{}", context_indicators.join(","))
        }
    }
    
    /// Extract context information from an update user request to determine if password checks can be skipped
    fn extract_context_from_update_request(request: &UpdateUserRequest) -> String {
        // Check if this appears to be a migration scenario
        // Indicators: presence of external_id or specific metadata
        let mut context_indicators = Vec::new();
        
        if request.external_id.is_some() {
            context_indicators.push("external_id");
        }
        
        if let Some(metadata) = &request.private_metadata {
            if metadata.to_string().contains("migration") || metadata.to_string().contains("import") {
                context_indicators.push("migration_metadata");
            }
        }
        
        if context_indicators.is_empty() {
            "regular_update".to_string()
        } else {
            format!("migration:{}", context_indicators.join(","))
        }
    }
    
    /// Validate password strength against security requirements
    fn validate_password_strength(password: &str) -> bool {
        // Check minimum length
        if password.len() < Self::MIN_PASSWORD_LENGTH {
            return false;
        }
        
        // Check for complexity requirements
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_number = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());
        
        // Password should meet at least 3 of the 4 complexity requirements
        let complexity_count = [has_lowercase, has_uppercase, has_number, has_special]
            .iter()
            .filter(|&&x| x)
            .count();
            
        complexity_count >= 3
    }
    /// Validate and log any security concerns in the user creation request
    pub fn validate_create_user_request(request: &CreateUserRequest) -> bool {
        // Check for potential security risks with password handling
        if let Some(skip_checks) = request.skip_password_checks {
            if skip_checks {
                // Log a warning about skipping password checks
                PasswordValidator::warn_on_skip_password_checks(Some("CreateUserRequest"));
                
                // Evaluate if this is a controlled migration context
                // Extract context from the request to determine if skip is allowed
                let context = Self::extract_context_from_request(request);
                
                // Only allow skipping in safe contexts
                return PasswordValidator::is_safe_to_skip_password_checks(&context);
            }
        }

        // Check for potential security risks with password requirement bypassing
        if let Some(skip_requirement) = request.skip_password_requirement {
            if skip_requirement {
                // Log a warning about skipping password requirement
                PasswordValidator::warn_on_skip_password_requirement(Some("CreateUserRequest"));
                
                // Evaluate if this is a controlled migration context
                // Extract context from the request to determine if skip is allowed
                let context = Self::extract_context_from_request(request);
                
                // Only allow skipping in safe contexts
                return PasswordValidator::is_safe_to_skip_password_requirement(&context);
            }
        }

        // Validate password strength if a password is provided and not skipping checks
        if let Some(Some(password)) = &request.password {
            if request.skip_password_checks.unwrap_or(false) == false {
                // Validate password strength
                return Self::validate_password_strength(password);
            }
        }

        true
    }

    /// Validate and log any security concerns in the user update request
    pub fn validate_update_user_request(request: &UpdateUserRequest) -> bool {
        // Check for potential security risks with password handling
        if let Some(skip_checks) = &request.skip_password_checks {
            if let Some(do_skip) = skip_checks {
                if *do_skip {
                    // Log a warning about skipping password checks
                    PasswordValidator::warn_on_skip_password_checks(Some("UpdateUserRequest"));
                    
                    // Evaluate if this is a controlled migration context
                    // Extract context from the request to determine if skip is allowed
                    let context = Self::extract_context_from_update_request(request);
                    
                    // Only allow skipping in safe contexts
                    return PasswordValidator::is_safe_to_skip_password_checks(&context);
                }
            }
        }

        // Validate password strength if a password is provided and not skipping checks
        if let Some(Some(password)) = &request.password {
            let is_skipping = match &request.skip_password_checks {
                Some(Some(skip)) => *skip,
                _ => false,
            };
            
            if !is_skipping {
                // Validate password strength
                return Self::validate_password_strength(password);
            }
        }

        true
    }
}