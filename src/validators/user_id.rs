use regex::Regex;

/// Validates a user ID to ensure it matches Clerk's expected format.
/// Clerk user IDs typically start with "user_" followed by a string of characters.
pub fn validate_user_id_format(user_id: &str) -> bool {
    // Based on observation of Clerk's format - user IDs start with "user_" followed by alphanumeric characters
    let re = Regex::new(r"^user_[a-zA-Z0-9]+$").unwrap();
    re.is_match(user_id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_user_id_format() {
        // Valid user IDs
        assert!(validate_user_id_format("user_1234567890"));
        assert!(validate_user_id_format("user_abcdef123456"));
        
        // Invalid user IDs
        assert!(!validate_user_id_format(""));
        assert!(!validate_user_id_format("user"));
        assert!(!validate_user_id_format("user_"));
        assert!(!validate_user_id_format("user-12345"));
        assert!(!validate_user_id_format("org_12345"));
    }
}