/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 */

use log::{warn, error, info};
use std::collections::HashSet;

/// Validates and logs warnings for potentially risky password operations
pub struct PasswordValidator;

/// Password validation result with specific error details
#[derive(Debug, PartialEq)]
pub enum PasswordValidationError {
    TooShort,
    MissingUppercase,
    MissingLowercase,
    MissingDigit,
    MissingSpecialChar,
    CommonPassword,
    Other(String),
}

impl PasswordValidator {
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
        // This implementation allows skipping only in controlled migration scenarios
        // Add implementation-specific logic here
        if context.contains("migration") || context.contains("import") {
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
        // This implementation allows skipping only in controlled migration scenarios
        // Add implementation-specific logic here
        if context.contains("migration") || context.contains("import") {
            true
        } else {
            error!("Attempted to skip password requirement in an unauthorized context: {}", context);
            false
        }
    }

    /// Validates a password against security requirements
    /// 
    /// Returns Ok(()) if the password meets all requirements, or Err with specific validation error
    pub fn validate_password(password: &str) -> Result<(), PasswordValidationError> {
        // Minimum length check (8 characters)
        if password.len() < 8 {
            return Err(PasswordValidationError::TooShort);
        }

        // Check for character diversity
        let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
        let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
        let has_digit = password.chars().any(|c| c.is_ascii_digit());
        let has_special = password.chars().any(|c| !c.is_alphanumeric());

        if !has_uppercase {
            return Err(PasswordValidationError::MissingUppercase);
        }

        if !has_lowercase {
            return Err(PasswordValidationError::MissingLowercase);
        }

        if !has_digit {
            return Err(PasswordValidationError::MissingDigit);
        }

        if !has_special {
            return Err(PasswordValidationError::MissingSpecialChar);
        }

        // Check against common passwords
        if Self::is_common_password(password) {
            return Err(PasswordValidationError::CommonPassword);
        }

        Ok(())
    }

    /// Checks if a password is in a list of commonly used (and therefore insecure) passwords
    fn is_common_password(password: &str) -> bool {
        // A small sample of common passwords for demonstration
        // In a production environment, this would be more comprehensive
        // and potentially use external services or databases
        let common_passwords: HashSet<&str> = [
            "password", "password123", "123456", "qwerty", "admin",
            "welcome", "letmein", "football", "iloveyou", "monkey",
        ].iter().cloned().collect();

        common_passwords.contains(&password.to_lowercase().as_str())
    }

    /// Get a human-readable message for a password validation error
    pub fn get_validation_error_message(error: &PasswordValidationError) -> String {
        match error {
            PasswordValidationError::TooShort => 
                "Password must be at least 8 characters long".to_string(),
            PasswordValidationError::MissingUppercase => 
                "Password must contain at least one uppercase letter".to_string(),
            PasswordValidationError::MissingLowercase => 
                "Password must contain at least one lowercase letter".to_string(),
            PasswordValidationError::MissingDigit => 
                "Password must contain at least one number".to_string(),
            PasswordValidationError::MissingSpecialChar => 
                "Password must contain at least one special character".to_string(),
            PasswordValidationError::CommonPassword => 
                "Password is too common and easily guessed".to_string(),
            PasswordValidationError::Other(msg) => msg.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_too_short() {
        let result = PasswordValidator::validate_password("Abc1!");
        assert!(matches!(result, Err(PasswordValidationError::TooShort)));
    }

    #[test]
    fn test_password_missing_uppercase() {
        let result = PasswordValidator::validate_password("password123!");
        assert!(matches!(result, Err(PasswordValidationError::MissingUppercase)));
    }

    #[test]
    fn test_password_missing_lowercase() {
        let result = PasswordValidator::validate_password("PASSWORD123!");
        assert!(matches!(result, Err(PasswordValidationError::MissingLowercase)));
    }

    #[test]
    fn test_password_missing_digit() {
        let result = PasswordValidator::validate_password("Password!");
        assert!(matches!(result, Err(PasswordValidationError::MissingDigit)));
    }

    #[test]
    fn test_password_missing_special_char() {
        let result = PasswordValidator::validate_password("Password123");
        assert!(matches!(result, Err(PasswordValidationError::MissingSpecialChar)));
    }

    #[test]
    fn test_password_common() {
        let result = PasswordValidator::validate_password("Password123!");
        assert!(matches!(result, Ok(())));

        let result = PasswordValidator::validate_password("Welcome123!");
        assert!(matches!(result, Err(PasswordValidationError::CommonPassword)));
    }

    #[test]
    fn test_valid_password() {
        let result = PasswordValidator::validate_password("Str0ng_P@ssw0rd!");
        assert!(matches!(result, Ok(())));
    }
}