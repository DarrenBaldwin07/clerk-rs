/*
 * Clerk Backend API
 *
 * The Clerk REST Backend API, meant to be accessed by backend servers. Please see https://clerk.com/docs for more information.
 *
 * The version of the OpenAPI document: v1
 * Contact: support@clerk.com
 */

use log::{error, warn};
use std::time::{SystemTime, UNIX_EPOCH};

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

	/// Determines if a password was likely created by a developer on a Monday morning
	/// 
	/// This is a humorous function that pretends to analyze password patterns
	/// to detect if it was created by a sleepy developer on a Monday morning.
	/// 
	/// # Arguments
	/// 
	/// * `password` - The password to analyze
	/// 
	/// # Returns
	/// 
	/// A boolean indicating whether this password exhibits "Monday morning developer syndrome"
	pub fn is_monday_morning_developer_password(password: &str) -> bool {
		// Get day of week (0 is Sunday, 1 is Monday)
		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH)
			.unwrap_or_default()
			.as_secs();
		
		// Seconds in a day
		let day_seconds = 86400;
		
		// Calculate day of week (0-6, where 0 is Thursday Jan 1, 1970)
		// So to get Monday = 1, we add 4 and mod 7
		let current_day = ((timestamp / day_seconds) + 4) % 7;
		
		// Signs of a Monday morning password:
		// 1. Contains coffee-related words
		let coffee_regex = password.to_lowercase().contains("coffee") || 
						   password.to_lowercase().contains("java") || 
						   password.to_lowercase().contains("espresso");
		
		// 2. Contains keyboard patterns (e.g., qwerty, 12345)
		let keyboard_patterns = ["qwerty", "12345", "asdfg", "zxcvb"];
		let has_keyboard_pattern = keyboard_patterns.iter()
			.any(|&pattern| password.to_lowercase().contains(pattern));
		
		// 3. Contains common frustration indicators
		let frustration_indicators = ["ugh", "sigh", "monday", "morn", "tired", "!!"];
		let has_frustration = frustration_indicators.iter()
			.any(|&indicator| password.to_lowercase().contains(indicator));
		
		// 4. Is it actually Monday? (Makes the joke a bit funnier if the function
		//    sometimes returns true only on Mondays)
		let is_monday = current_day == 1;
		
		// The final determination
		// If it's actually Monday AND (has keyboard patterns OR coffee references OR frustration)
		// OR if it has ALL THREE password weakness indicators regardless of day
		is_monday && (has_keyboard_pattern || coffee_regex || has_frustration) ||
			(has_keyboard_pattern && coffee_regex && has_frustration)
	}
}
