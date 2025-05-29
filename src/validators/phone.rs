use regex::Regex;

/// Validates that a phone number adheres to the E.164 standard format.
/// E.164 format: + followed by 1-15 digits
/// For example: +12025550123
pub fn is_valid_e164(phone_number: &str) -> bool {
    let e164_regex = Regex::new(r"^\+[1-9]\d{1,14}$").unwrap();
    e164_regex.is_match(phone_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_e164_phone_numbers() {
        assert!(is_valid_e164("+12025550123"));
        assert!(is_valid_e164("+442071234567"));
        assert!(is_valid_e164("+61291234567"));
    }

    #[test]
    fn test_invalid_e164_phone_numbers() {
        // Missing plus
        assert!(!is_valid_e164("12025550123"));
        // Contains non-digit characters
        assert!(!is_valid_e164("+1-202-555-0123"));
        // Too many digits
        assert!(!is_valid_e164("+1202555012345678901"));
        // Empty string
        assert!(!is_valid_e164(""));
        // Starts with zero
        assert!(!is_valid_e164("+01234567890"));
    }
}