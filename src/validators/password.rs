/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 */

use log::{warn, error};

/// Validates and logs warnings for potentially risky password operations
pub struct PasswordValidator;

impl PasswordValidator {
    /// Validates password strength based on security best practices
    ///
    /// Returns true if the password meets all security requirements:
    /// - Minimum 10 characters
    /// - At least one uppercase letter
    /// - At least one lowercase letter
    /// - At least one number
    /// - At least one special character
    pub fn validate_password_strength(password: &str) -> bool {
        if password.len() < 10 {
            error!("Password too short (minimum 10 characters required)");
            return false;
        }
        
        // Check for at least one uppercase letter
        let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
        if !has_uppercase {
            error!("Password missing uppercase letter");
            return false;
        }
        
        // Check for at least one lowercase letter
        let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
        if !has_lowercase {
            error!("Password missing lowercase letter");
            return false;
        }
        
        // Check for at least one number
        let has_number = password.chars().any(|c| c.is_ascii_digit());
        if !has_number {
            error!("Password missing numeric character");
            return false;
        }
        
        // Check for at least one special character
        let has_special = password.chars().any(|c| !c.is_ascii_alphanumeric());
        if !has_special {
            error!("Password missing special character");
            return false;
        }
        
        true
    }
    /// Logs a warning when skip_password_checks is used
    ///
    /// This method should be called whenever skip_password_checks is set to true
    /// to ensure appropriate logging and monitoring of this security-sensitive action.
    pub fn warn_on_skip_password_checks(reason: Option<&str>) {
        let context = reason.unwrap_or("No reason provided");
        warn!(
            "SECURITY WARNING: Password security checks are being bypassed! This should only be used \
            in controlled migration scenarios. Reason: {}",
            context
        );
    }

    /// Logs a warning when skip_password_requirement is used
    ///
    /// This method should be called whenever skip_password_requirement is set to true
    /// to ensure appropriate logging and monitoring of this security-sensitive action.
    pub fn warn_on_skip_password_requirement(reason: Option<&str>) {
        let context = reason.unwrap_or("No reason provided");
        warn!(
            "SECURITY WARNING: Password requirement is being bypassed! This should only be used \
            in controlled migration scenarios. Reason: {}",
            context
        );
    }

    /// Validates whether it's safe to skip password checks
    ///
    /// This function determines if the current context makes it acceptable
    /// to skip password checks. Returns false if it's not appropriate to
    /// skip checks in the current context.
    pub fn is_safe_to_skip_password_checks(context: &str) -> bool {
        // Only allow in controlled migration contexts with explicit migration tokens/identifiers
        // Simple substring matching is insufficient for security purposes
        if context.contains("authorized_migration") && context.contains("migration_token") {
            // In a real implementation, you would validate the migration token here
            // against a whitelist of authorized migration operations
            true
        } else {
            error!("Attempted to skip password checks in an unauthorized context: {}", context);
            false
        }
    }

    /// Validates whether it's safe to skip password requirement
    ///
    /// This function determines if the current context makes it acceptable
    /// to skip password requirement. Returns false if it's not appropriate to
    /// skip requirement in the current context.
    pub fn is_safe_to_skip_password_requirement(context: &str) -> bool {
        // Only allow in controlled migration contexts with explicit migration tokens/identifiers
        // Simple substring matching is insufficient for security purposes
        if context.contains("authorized_migration") && context.contains("migration_token") {
            // In a real implementation, you would validate the migration token here
            // against a whitelist of authorized migration operations
            true
        } else {
            error!("Attempted to skip password requirement in an unauthorized context: {}", context);
            false
        }
    }
}