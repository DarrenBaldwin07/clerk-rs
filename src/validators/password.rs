/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 */

use log::{warn, error};
use crate::models::create_user_request::PasswordHasher;

/// Validates and logs warnings for potentially risky password operations
pub struct PasswordValidator;

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

    /// Logs a warning when an insecure password hashing algorithm is used
    ///
    /// This method should be called whenever an insecure algorithm like MD5 or SHA256
    /// is specified. These algorithms will be transparently migrated to Bcrypt on first sign-in.
    pub fn warn_on_insecure_password_hasher(hasher: &PasswordHasher, reason: Option<&str>) {
        let context = reason.unwrap_or("No reason provided");
        match hasher {
            PasswordHasher::Md5 => {
                warn!(
                    "SECURITY WARNING: MD5 is cryptographically insecure and should not be used for password hashing. \
                    It will be automatically upgraded to Bcrypt on the user's first successful sign-in. \
                    Reason for use: {}",
                    context
                );
            },
            PasswordHasher::Sha256 => {
                warn!(
                    "SECURITY WARNING: SHA256 without salt/iterations is cryptographically insecure and should not be used \
                    for password hashing. It will be automatically upgraded to Bcrypt on the user's first successful sign-in. \
                    Reason for use: {}",
                    context
                );
            },
            PasswordHasher::Phpass => {
                warn!(
                    "SECURITY WARNING: phpass is based on MD5 which is insecure and should not be used for password hashing. \
                    Consider using Argon2id, Bcrypt, or PBKDF2 instead. \
                    Reason for use: {}",
                    context
                );
            },
            PasswordHasher::Pbkdf2Sha1 => {
                warn!(
                    "SECURITY WARNING: SHA1 is considered weak and should not be used for password hashing. \
                    Consider using pbkdf2_sha256 instead. \
                    Reason for use: {}",
                    context
                );
            },
            _ => {} // No warning for secure algorithms
        }
    }

    /// Checks if the provided password hasher is considered secure
    ///
    /// Returns true for secure algorithms and false for deprecated/insecure ones
    pub fn is_secure_password_hasher(hasher: &PasswordHasher) -> bool {
        match hasher {
            PasswordHasher::Argon2i => true,
            PasswordHasher::Argon2id => true,
            PasswordHasher::Bcrypt => true,
            PasswordHasher::BcryptSha256Django => true,
            PasswordHasher::Pbkdf2Sha256 => true,
            PasswordHasher::Pbkdf2Sha256Django => true,
            PasswordHasher::ScryptFirebase => true,
            PasswordHasher::Md5 => false,
            PasswordHasher::Sha256 => false,
            PasswordHasher::Phpass => false,
            PasswordHasher::Pbkdf2Sha1 => false,
        }
    }

    /// Validates whether it's safe to use an insecure password hasher
    ///
    /// This function determines if the current context makes it acceptable
    /// to use an insecure algorithm. Returns false if it's not appropriate.
    pub fn is_safe_to_use_insecure_hasher(hasher: &PasswordHasher, context: &str) -> bool {
        // Always allow secure algorithms
        if Self::is_secure_password_hasher(hasher) {
            return true;
        }

        // Only allow insecure algorithms in controlled migration scenarios
        if context.contains("migration") || context.contains("import") {
            // Log a warning but allow it
            Self::warn_on_insecure_password_hasher(hasher, Some(context));
            true
        } else {
            error!(
                "Attempted to use insecure password hasher {:?} in an unauthorized context: {}",
                hasher, context
            );
            false
        }
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
}