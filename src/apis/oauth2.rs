use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::time::{Duration, SystemTime};

/// A Token represents the credentials used to authorize requests to the Clerk API.
#[derive(Debug, Clone)]
pub struct Token {
    /// The access token itself.
    pub access_token: String,
    /// The token type, typically "Bearer".
    pub token_type: Option<String>,
    /// The refresh token, if available.
    pub refresh_token: Option<String>,
    /// The expiration time of the token, if available.
    pub expiry: Option<SystemTime>,
}

impl Token {
    /// Creates a new Token with the given access token.
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: Some("Bearer".to_string()),
            refresh_token: None,
            expiry: None,
        }
    }

    /// Creates a new Token with the given access token and expiry.
    pub fn new_with_expiry(access_token: String, expires_in: u64) -> Self {
        let expiry = SystemTime::now().checked_add(Duration::from_secs(expires_in));
        
        Self {
            access_token,
            token_type: Some("Bearer".to_string()),
            refresh_token: None,
            expiry,
        }
    }

    /// Creates a new Token with the given access token, expiry, and refresh token.
    pub fn new_with_refresh(access_token: String, expires_in: u64, refresh_token: String) -> Self {
        let expiry = SystemTime::now().checked_add(Duration::from_secs(expires_in));
        
        Self {
            access_token,
            token_type: Some("Bearer".to_string()),
            refresh_token: Some(refresh_token),
            expiry,
        }
    }

    /// Check if the token is valid (not expired).
    pub fn is_valid(&self) -> bool {
        if self.access_token.is_empty() {
            return false;
        }
        
        if let Some(expiry) = self.expiry {
            // Add a 10-second buffer before expiration to avoid edge cases
            match expiry.checked_sub(Duration::from_secs(10)) {
                Ok(expiry_with_delta) => SystemTime::now() < expiry_with_delta,
                Err(_) => SystemTime::now() < expiry,
            }
        } else {
            // No expiry time means not expired
            true
        }
    }

    /// Set the Authorization header with the token.
    pub fn set_auth_header(&self, headers: &mut HeaderMap) -> Result<(), Box<dyn Error>> {
        let token_type = self.token_type.as_deref().unwrap_or("Bearer");
        let auth_value = format!("{} {}", token_type, self.access_token);
        
        headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(&auth_value)?,
        );
        
        Ok(())
    }
}

/// A TokenSource is anything that can return a token.
pub trait TokenSource: Send + Sync {
    /// Returns a valid Token or an error if a Token couldn't be acquired.
    fn token(&self) -> Result<Token, Box<dyn Error + Send + Sync>>;
}

/// StaticTokenSource returns the same Token every time.
#[derive(Debug, Clone)]
pub struct StaticTokenSource {
    token: Token,
}

impl StaticTokenSource {
    /// Creates a new StaticTokenSource with the given token.
    pub fn new(token: Token) -> Self {
        Self { token }
    }
}

impl TokenSource for StaticTokenSource {
    fn token(&self) -> Result<Token, Box<dyn Error + Send + Sync>> {
        Ok(self.token.clone())
    }
}

/// ReuseTokenSource caches the Token obtained from the source.
pub struct ReuseTokenSource {
    token: Arc<Mutex<Option<Token>>>,
    source: Box<dyn TokenSource>,
}

impl ReuseTokenSource {
    /// Creates a new ReuseTokenSource with the given initial token and source.
    pub fn new(initial_token: Option<Token>, source: Box<dyn TokenSource>) -> Self {
        Self {
            token: Arc::new(Mutex::new(initial_token)),
            source,
        }
    }
}

impl TokenSource for ReuseTokenSource {
    fn token(&self) -> Result<Token, Box<dyn Error + Send + Sync>> {
        let mut token_guard = match self.token.lock() {
            Ok(guard) => guard,
            Err(e) => return Err(format!("Failed to acquire lock: {}", e).into()),
        };
        
        // Return existing token if valid
        if let Some(token) = token_guard.as_ref() {
            if token.is_valid() {
                return Ok(token.clone());
            }
        }
        
        // Get new token from source
        let new_token = self.source.token()?;
        *token_guard = Some(new_token.clone());
        Ok(new_token)
    }
}