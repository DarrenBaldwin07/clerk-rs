#[cfg(test)]
mod tests {
    use crate::validators::input::{validate_str_param, validate_request, ValidationError};
    use serde::Serialize;

    #[test]
    fn test_validate_str_param_success() {
        let result = validate_str_param("test_param", "valid_value");
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_str_param_empty() {
        let result = validate_str_param("test_param", "");
        assert!(result.is_err());
        match result {
            Err(ValidationError::EmptyOrNull(param)) => assert_eq!(param, "test_param"),
            _ => panic!("Expected EmptyOrNull error"),
        }
    }

    #[test]
    fn test_validate_str_param_whitespace() {
        let result = validate_str_param("test_param", "   ");
        assert!(result.is_err());
        match result {
            Err(ValidationError::EmptyOrNull(param)) => assert_eq!(param, "test_param"),
            _ => panic!("Expected EmptyOrNull error"),
        }
    }

    #[derive(Serialize)]
    struct TestRequest {
        field: String,
    }

    #[test]
    fn test_validate_request_success() {
        let request = Some(TestRequest { field: "test".to_string() });
        let result = validate_request(&request);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_request_none() {
        let request: Option<TestRequest> = None;
        let result = validate_request(&request);
        assert!(result.is_err());
        match result {
            Err(ValidationError::EmptyOrNull(param)) => assert_eq!(param, "request"),
            _ => panic!("Expected EmptyOrNull error"),
        }
    }
}