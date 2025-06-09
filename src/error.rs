use reqwest::StatusCode;
use std::{error, fmt};

/// A structured representation of a Clerk API error
#[derive(Debug, Clone)]
pub struct ApiError {
    /// HTTP status code from the response
    pub status: StatusCode,
    /// Raw content from the error response
    pub content: String,
    /// The parsed ClerkErrors if available
    pub clerk_errors: Option<crate::models::ClerkErrors>,
    /// Additional context about the error (e.g., original JSON value when parsing fails)
    pub context: Option<serde_json::Value>,
}

impl ApiError {
    /// Creates a new ApiError from the raw response
    pub fn new(status: StatusCode, content: String) -> Self {
        let clerk_errors: Option<crate::models::ClerkErrors> = serde_json::from_str(&content).ok();
        
        // If we couldn't parse the response as ClerkErrors, try to parse it as a JSON value
        // to retain as much information as possible
        let context = if clerk_errors.is_none() {
            serde_json::from_str(&content).ok()
        } else {
            None
        };

        ApiError {
            status,
            content,
            clerk_errors,
            context,
        }
    }

    /// Returns the error message if available
    pub fn message(&self) -> Option<String> {
        self.clerk_errors.as_ref().and_then(|e| {
            if !e.errors.is_empty() {
                Some(e.errors[0].message.clone())
            } else {
                None
            }
        })
    }

    /// Returns the error code if available
    pub fn code(&self) -> Option<String> {
        self.clerk_errors.as_ref().and_then(|e| {
            if !e.errors.is_empty() {
                Some(e.errors[0].code.clone())
            } else {
                None
            }
        })
    }

    /// Returns true if this error matches the given status code
    pub fn status_is(&self, code: StatusCode) -> bool {
        self.status == code
    }
}

/// The main error type for the Clerk API client
#[derive(Debug)]
pub enum Error {
    /// An error from the reqwest HTTP client
    Reqwest(reqwest::Error),
    /// An error when serializing or deserializing JSON
    Serde(serde_json::Error),
    /// An I/O error
    Io(std::io::Error),
    /// An error response from the Clerk API
    ApiError(ApiError),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API error (status {})", self.status)?;
        
        if let Some(msg) = self.message() {
            write!(f, ": {}", msg)?;
        }
        
        if let Some(code) = self.code() {
            write!(f, " (code: {})", code)?;
        }
        
        Ok(())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Reqwest(e) => write!(f, "HTTP request error: {}", e),
            Error::Serde(e) => write!(f, "JSON serialization error: {}", e),
            Error::Io(e) => write!(f, "I/O error: {}", e),
            Error::ApiError(e) => write!(f, "{}", e),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Error::Reqwest(e) => Some(e),
            Error::Serde(e) => Some(e),
            Error::Io(e) => Some(e),
            Error::ApiError(_) => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Reqwest(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Self {
        Error::Serde(e)
    }
}

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::Io(e)
    }
}

impl From<ApiError> for Error {
    fn from(e: ApiError) -> Self {
        Error::ApiError(e)
    }
}