/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 */

use log::error;
use std::io;

/// Validator for OAuth-related inputs
pub struct OAuthValidator;

impl OAuthValidator {
    /// Validates an OAuth application ID
    ///
    /// This method checks that the OAuth application ID:
    /// - Is not empty
    /// - Does not contain invalid characters
    /// - Meets the expected format requirements
    ///
    /// Returns Ok(()) if valid, or an appropriate error otherwise.
    pub fn validate_application_id(oauth_application_id: &str) -> Result<(), io::Error> {
        // Check for empty string
        if oauth_application_id.is_empty() {
            error!("OAuth application ID cannot be empty");
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "OAuth application ID cannot be empty",
            ));
        }

        // Check for valid characters (alphanumeric and specific symbols)
        // Typically OAuth application IDs are alphanumeric with possible underscore or dash
        if !oauth_application_id
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            error!("OAuth application ID contains invalid characters");
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "OAuth application ID contains invalid characters. Only alphanumeric characters, underscores, and dashes are allowed.",
            ));
        }

        // Additional checks can be added here (e.g., length constraints)
        if oauth_application_id.len() < 5 {
            error!("OAuth application ID is too short");
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "OAuth application ID is too short (minimum 5 characters required).",
            ));
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_oauth_application_id() {
        let result = OAuthValidator::validate_application_id("app_123456789");
        assert!(result.is_ok());
    }

    #[test]
    fn test_empty_oauth_application_id() {
        let result = OAuthValidator::validate_application_id("");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_chars_oauth_application_id() {
        let result = OAuthValidator::validate_application_id("app/123");
        assert!(result.is_err());
    }

    #[test]
    fn test_too_short_oauth_application_id() {
        let result = OAuthValidator::validate_application_id("app");
        assert!(result.is_err());
    }
}