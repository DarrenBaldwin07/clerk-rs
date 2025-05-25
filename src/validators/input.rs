use regex::Regex;

/// Validates that a session ID is present and has the correct format.
/// Clerk session IDs typically follow the format "sess_" followed by a string of alphanumeric characters.
pub fn validate_session_id(session_id: &str) -> Result<(), String> {
    if session_id.is_empty() {
        return Err("Session ID cannot be empty".to_string());
    }

    // Validate that session_id follows the expected format: sess_<alphanumeric>
    let re = Regex::new(r"^sess_[a-zA-Z0-9]+$").unwrap();
    if !re.is_match(session_id) {
        return Err("Invalid session ID format. Expected format: sess_<alphanumeric>".to_string());
    }

    Ok(())
}

/// Validates that a template name is present and not empty.
pub fn validate_template_name(template_name: &str) -> Result<(), String> {
    if template_name.is_empty() {
        return Err("Template name cannot be empty".to_string());
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_session_id_valid() {
        let result = validate_session_id("sess_123456abcdef");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_session_id_empty() {
        let result = validate_session_id("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Session ID cannot be empty");
    }

    #[test]
    fn test_validate_session_id_invalid_format() {
        let result = validate_session_id("invalid_id");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid session ID format. Expected format: sess_<alphanumeric>");
    }

    #[test]
    fn test_validate_session_id_invalid_chars() {
        let result = validate_session_id("sess_123!@#");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Invalid session ID format. Expected format: sess_<alphanumeric>");
    }

    #[test]
    fn test_validate_template_name_valid() {
        let result = validate_template_name("my_template");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_template_name_empty() {
        let result = validate_template_name("");
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Template name cannot be empty");
    }
}