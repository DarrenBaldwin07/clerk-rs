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
use log::error;

/// Middleware for validating user creation parameters
pub struct UsersMiddleware;

impl UsersMiddleware {
    /// Validate and log any security concerns in the user creation request
    pub fn validate_create_user_request(request: &CreateUserRequest) -> bool {
        // Check for potential security risks with password handling
        if let Some(skip_checks) = request.skip_password_checks {
            if skip_checks {
                // Always log a warning about skipping password checks
                PasswordValidator::warn_on_skip_password_checks(Some("CreateUserRequest"));
                
                // Strictly validate if this is a legitimate migration context
                // Extract context from the request metadata or other attributes
                let context = if let Some(metadata) = &request.private_metadata {
                    if let Some(migration_info) = metadata.get("migration_info") {
                        migration_info.to_string()
                    } else {
                        "No migration info provided".to_string()
                    }
                } else {
                    "No context provided".to_string()
                };
                
                // Enforce strict validation to ensure this is only used for authorized migrations
                if !PasswordValidator::is_safe_to_skip_password_checks(&context) {
                    error!("Password security checks skipping denied - insufficient authorization");
                    return false;
                }
            }
        }

        // Check for potential security risks with password requirement bypassing
        if let Some(skip_requirement) = request.skip_password_requirement {
            if skip_requirement {
                // Always log a warning about skipping password requirement
                PasswordValidator::warn_on_skip_password_requirement(Some("CreateUserRequest"));
                
                // Extract context from the request metadata or other attributes
                let context = if let Some(metadata) = &request.private_metadata {
                    if let Some(migration_info) = metadata.get("migration_info") {
                        migration_info.to_string()
                    } else {
                        "No migration info provided".to_string()
                    }
                } else {
                    "No context provided".to_string()
                };
                
                // Enforce strict validation to ensure this is only used for authorized migrations
                if !PasswordValidator::is_safe_to_skip_password_requirement(&context) {
                    error!("Password requirement skipping denied - insufficient authorization");
                    return false;
                }
            }
        }

        // Validate password strength if a plaintext password is provided
        if let Some(Some(password)) = &request.password {
            if !PasswordValidator::validate_password_strength(password) {
                error!("Password does not meet minimum security requirements");
                return false;
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
                    // Always log a warning about skipping password checks
                    PasswordValidator::warn_on_skip_password_checks(Some("UpdateUserRequest"));
                    
                    // Extract context from the request metadata or other attributes
                    let context = if let Some(metadata) = &request.private_metadata {
                        if let Some(migration_info) = metadata.get("migration_info") {
                            migration_info.to_string()
                        } else {
                            "No migration info provided".to_string()
                        }
                    } else {
                        "No context provided".to_string()
                    };
                    
                    // Enforce strict validation to ensure this is only used for authorized migrations
                    if !PasswordValidator::is_safe_to_skip_password_checks(&context) {
                        error!("Password security checks skipping denied - insufficient authorization");
                        return false;
                    }
                }
            }
        }

        // Validate password strength if a plaintext password is provided
        if let Some(Some(password)) = &request.password {
            if !PasswordValidator::validate_password_strength(password) {
                error!("Password does not meet minimum security requirements");
                return false;
            }
        }

        true
    }
    
}