/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 */

use bcrypt::{hash, verify, DEFAULT_COST};
use log::{error, warn};

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

	/// Securely hashes a password using bcrypt
	///
	/// This method should be used whenever a password needs to be stored.
	/// It applies bcrypt hashing with a secure cost factor.
	pub fn hash_password(password: &str) -> Result<String, String> {
		hash(password, DEFAULT_COST).map_err(|e| format!("Password hashing error: {}", e))
	}

	/// Verifies a password against a stored hash
	///
	/// This method should be used to validate a user's password by comparing
	/// the provided plain text password with a previously stored hash.
	pub fn verify_password(password: &str, hash: &str) -> Result<bool, String> {
		verify(password, hash).map_err(|e| format!("Password verification error: {}", e))
	}
}
