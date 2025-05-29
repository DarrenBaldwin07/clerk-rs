use regex::Regex;

/// Validates an email address format
pub fn validate_email(email: &str) -> bool {
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    email_regex.is_match(email)
}

/// Validates that a string is not empty
pub fn validate_non_empty(value: &str) -> bool {
    !value.trim().is_empty()
}

/// Validates that a user ID is in the correct format
/// Clerk user IDs typically start with "user_" followed by alphanumeric characters
pub fn validate_user_id(user_id: &str) -> bool {
    let user_id_regex = Regex::new(r"^user_[a-zA-Z0-9]+$").unwrap();
    user_id_regex.is_match(user_id)
}

/// Validates organization role is one of the allowed values
pub fn validate_role(role: &str) -> bool {
    // Basic validation to ensure role is not empty
    // This could be expanded to include specific role validation if needed
    validate_non_empty(role)
}