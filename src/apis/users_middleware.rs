/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 */

use log::{error, warn};
use crate::models::{CreateUserRequest, UpdateUserRequest};
use crate::validators::password::{PasswordValidator, PasswordValidationError};

/// Middleware for validating user creation parameters
pub struct UsersMiddleware;

impl UsersMiddleware {
    /// Validate and log any security concerns in the user creation request
    pub fn validate_create_user_request(request: &CreateUserRequest) -> bool {
        // Check for potential security risks with password handling
        if let Some(skip_checks) = request.skip_password_checks {
            if skip_checks {
                // Log a warning about skipping password checks
                PasswordValidator::warn_on_skip_password_checks(Some("CreateUserRequest"));
                
                // Evaluate if this is a controlled migration context
                // This is a stub - in a real implementation, you might check additional context
                // like headers, metadata, or other request attributes
                return true;
            }
        }

        // Check for potential security risks with password requirement bypassing
        if let Some(skip_requirement) = request.skip_password_requirement {
            if skip_requirement {
                // Log a warning about skipping password requirement
                PasswordValidator::warn_on_skip_password_requirement(Some("CreateUserRequest"));
                
                // Evaluate if this is a controlled migration context
                // This is a stub - in a real implementation, you might check additional context
                return true;
            }
        }

        // Validate password if provided and not skipping checks
        if let Some(Some(password)) = &request.password {
            // If skip_password_checks is explicitly set to false or not set at all
            let should_validate = request.skip_password_checks.unwrap_or(false) == false;
            
            if should_validate {
                match PasswordValidator::validate_password(password) {
                    Ok(()) => {
                        // Password meets all requirements
                        return true;
                    },
                    Err(validation_error) => {
                        // Password failed validation
                        let error_message = PasswordValidator::get_validation_error_message(&validation_error);
                        error!("Password validation failed: {}", error_message);
                        return false;
                    }
                }
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
                    // This is a stub - in a real implementation, you might check additional context
                    return true;
                }
            }
        }

        // Validate password if provided and not skipping checks
        if let Some(password) = &request.password {
            if let Some(pw_value) = password {
                // Determine if we should validate based on skip_password_checks
                let should_validate = match &request.skip_password_checks {
                    Some(Some(skip)) => !skip,
                    _ => true, // Default to validation if not specified
                };

                if should_validate {
                    match PasswordValidator::validate_password(pw_value) {
                        Ok(()) => {
                            // Password meets all requirements
                            return true;
                        },
                        Err(validation_error) => {
                            // Password failed validation
                            let error_message = PasswordValidator::get_validation_error_message(&validation_error);
                            error!("Password validation failed: {}", error_message);
                            return false;
                        }
                    }
                }
            }
        }

        true
    }
}