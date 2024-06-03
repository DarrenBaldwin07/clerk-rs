use crate::{
	apis::jwks_api::{Jwks, JwksModel},
	clerk::Clerk,
};
use jsonwebtoken::{decode, decode_header, errors::Error as jwtError, Algorithm, DecodingKey, Header, Validation};
use std::{error::Error, fmt};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ClerkJwt {
	pub azp: Option<String>,
	pub exp: i32,
	pub iat: i32,
	pub iss: String,
	pub nbf: i32,
	pub sid: Option<String>,
	pub sub: String,
}

pub trait ClerkRequest {
	fn get_header(&self, key: &str) -> Option<String>;
	fn get_cookie(&self, key: &str) -> Option<String>;
}

#[derive(Debug)]
pub enum ClerkError {
	Unauthorized(String),
	InternalServerError(String),
}

impl fmt::Display for ClerkError {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		match self {
			ClerkError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
			ClerkError::InternalServerError(msg) => write!(f, "Internal Server Error: {}", msg),
		}
	}
}

impl Error for ClerkError {}

#[derive(Clone)]
pub struct ClerkAuthorizer {
	clerk_client: Clerk,
	validate_session_cookie: bool,
}

impl ClerkAuthorizer {
	/// Creates a Clerk authorizer
	pub fn new(clerk_client: Clerk, validate_session_cookie: bool) -> Self {
		Self {
			clerk_client,
			validate_session_cookie,
		}
	}

	/// Authorizes a service request against the Clerk auth provider
	pub async fn authorize<T>(&self, request: &T) -> Result<ClerkJwt, ClerkError>
	where
		T: ClerkRequest,
	{
		let jwks = match Jwks::get_jwks(&self.clerk_client).await {
			Ok(val) => val,
			Err(_) => return Err(ClerkError::InternalServerError(String::from("Error: Could not fetch JWKS!"))),
		};

		let access_token: String = match request.get_header("Authorization") {
			Some(val) => val.to_string().replace("Bearer ", ""),
			None => match self.validate_session_cookie {
				true => match request.get_cookie("__session") {
					Some(cookie) => cookie.to_string(),
					None => {
						return Err(ClerkError::Unauthorized(String::from(
							"Error: No Authorization header or session cookie found on the request payload!",
						)))
					}
				},
				false => {
					return Err(ClerkError::Unauthorized(String::from(
						"Error: No Authorization header found on the request payload!",
					)))
				}
			},
		};

		validate_jwt(&access_token, jwks)
	}
}

/// Validates a jwt token using a jwks
pub fn validate_jwt(token: &str, jwks: JwksModel) -> Result<ClerkJwt, ClerkError> {
	// If we were not able to parse the kid field we want to output an invalid case...
	let kid = match get_token_header(token).map(|h| h.kid) {
		Ok(Some(kid)) => kid,
		_ => {
			return Err(ClerkError::Unauthorized(String::from("Error: Invalid JWT!")));
		}
	};

	let jwk = jwks.keys.iter().find(|k| k.kid == kid);

	// Check to see if we found a valid jwk key with the token kid
	if let Some(j) = jwk {
		match j.alg.as_str() {
			// Currently, clerk only supports Rs256 by default
			"RS256" => {
				let decoding_key = DecodingKey::from_rsa_components(&j.n, &j.e)
					.map_err(|_| ClerkError::InternalServerError(String::from("Error: Invalid decoding key")))?;
				let mut validation = Validation::new(Algorithm::RS256);
				validation.validate_exp = true;
				validation.validate_nbf = true;

				match decode::<ClerkJwt>(token, &decoding_key, &validation) {
					Ok(token) => Ok(token.claims),
					_ => Err(ClerkError::Unauthorized(String::from("Error: Invalid JWT!"))),
				}
			}
			_ => Err(ClerkError::InternalServerError(String::from("Error: Unsupported key algorithm"))),
		}
	} else {
		// In the event that a matching jwk was not found we want to output an error
		Err(ClerkError::Unauthorized(String::from("Error: Invalid JWT!")))
	}
}

/// Extract the header from a jwt token
fn get_token_header(token: &str) -> Result<Header, jwtError> {
	let header = decode_header(&token);
	header
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::apis::jwks_api::JwksKey;
	use base64::engine::general_purpose::URL_SAFE_NO_PAD;
	use base64::prelude::*;
	use jsonwebtoken::{encode, errors::ErrorKind, Algorithm, EncodingKey, Header};
	use rsa::{pkcs1::EncodeRsaPrivateKey, traits::PublicKeyParts, RsaPrivateKey};
	use std::time::{SystemTime, UNIX_EPOCH};

	#[derive(Debug, serde::Serialize)]
	struct Claims {
		sub: String,
		iat: usize,
		nbf: usize,
		exp: usize,
		azp: String,
		iss: String,
		sid: String,
	}

	struct Helper {
		private_key: RsaPrivateKey,
	}

	impl Helper {
		pub fn new() -> Self {
			let mut rng = rand::thread_rng();

			Self {
				private_key: RsaPrivateKey::new(&mut rng, 2048).unwrap(),
			}
		}

		pub fn generate_jwt_token(&self, kid: Option<&str>, current_time: Option<usize>, expired: bool) -> String {
			let pem = self.private_key.to_pkcs1_pem(rsa::pkcs8::LineEnding::LF).unwrap();
			let encoding_key = EncodingKey::from_rsa_pem(pem.as_bytes()).expect("Failed to load encoding key");

			let current_time = current_time.unwrap_or(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize);

			let mut expiration = current_time;
			if !expired {
				expiration += 3600;
			}

			let claims = Claims {
				azp: "client_id".to_string(),
				sub: "user".to_string(),
				iat: current_time,
				exp: expiration,
				iss: "issuer".to_string(),
				nbf: current_time,
				sid: "session_id".to_string(),
			};

			let mut header = Header::new(Algorithm::RS256);
			if let Some(kid_value) = kid {
				header.kid = Some(kid_value.to_string());
			}

			let token = encode(&header, &claims, &encoding_key).expect("Failed to create jwt token");

			token
		}

		pub fn get_modulus_and_public_exponent(&self) -> (String, String) {
			let encoded_modulus = URL_SAFE_NO_PAD.encode(self.private_key.n().to_bytes_be().as_slice());
			let encoded_exponent = URL_SAFE_NO_PAD.encode(self.private_key.e().to_bytes_be().as_slice());
			(encoded_modulus, encoded_exponent)
		}
	}

	#[test]
	fn test_validate_jwt_success() {
		let helper = Helper::new();

		let kid = "bc63c2e9-5d1c-4e32-9b62-178f60409abd";

		let (modulus, exponent) = helper.get_modulus_and_public_exponent();

		let jwks_key = JwksKey {
			use_key: String::new(),
			kty: String::new(),
			kid: kid.to_string(),
			alg: String::from("RS256"),
			n: modulus,
			e: exponent,
		};
		let jwks = JwksModel { keys: vec![jwks_key] };

		let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
		let token = helper.generate_jwt_token(Some(kid), Some(current_time), false);

		let expected = ClerkJwt {
			azp: Some("client_id".to_string()),
			sub: "user".to_string(),
			iat: current_time as i32,
			exp: (current_time + 3600) as i32,
			iss: "issuer".to_string(),
			nbf: current_time as i32,
			sid: "session_id".to_string(),
		};

		match validate_jwt(token.as_str(), jwks) {
			Ok(jwt) => {
				assert_eq!(jwt, expected)
			}
			Err(_) => unreachable!("Unexpected invalid jwt token"),
		}
	}

	#[test]
	fn test_validate_jwt_error_getting_token_header() {
		let helper = Helper::new();

		let kid = "bc63c2e9-5d1c-4e32-9b62-178f60409abd";

		let (modulus, exponent) = helper.get_modulus_and_public_exponent();

		let jwks_key = JwksKey {
			use_key: String::new(),
			kty: String::new(),
			kid: kid.to_string(),
			alg: String::from("RS256"),
			n: modulus,
			e: exponent,
		};
		let jwks = JwksModel { keys: vec![jwks_key] };

		match validate_jwt("invalid_token", jwks) {
			Ok(_) => unreachable!("Unexpected valid jwt token"),
			Err(_) => assert!(true),
		}
	}

	#[test]
	fn test_validate_jwt_missing_token_kid() {
		let helper = Helper::new();

		let kid = "bc63c2e9-5d1c-4e32-9b62-178f60409abd";

		let (modulus, exponent) = helper.get_modulus_and_public_exponent();

		let jwks_key = JwksKey {
			use_key: String::new(),
			kty: String::new(),
			kid: kid.to_string(),
			alg: String::from("RS256"),
			n: modulus,
			e: exponent,
		};
		let jwks = JwksModel { keys: vec![jwks_key] };

		let token = helper.generate_jwt_token(None, None, false);

		assert!(matches!(validate_jwt(&token, jwks), Err(ClerkError::Unauthorized(_))))
	}

	#[test]
	fn test_validate_jwt_unmatched_jwks_key() {
		let helper = Helper::new();

		let (modulus, exponent) = helper.get_modulus_and_public_exponent();

		let jwks_key = JwksKey {
			use_key: String::new(),
			kty: String::new(),
			kid: String::from("a288cbf5-fec1-41e3-ae83-5b0d122bf925"),
			alg: String::from("RS256"),
			n: modulus,
			e: exponent,
		};
		let jwks = JwksModel { keys: vec![jwks_key] };

		let token = helper.generate_jwt_token(Some("bc63c2e9-5d1c-4e32-9b62-178f60409abd"), None, false);

		match validate_jwt(token.as_str(), jwks) {
			Ok(_) => unreachable!("Unexpected valid jwt token"),
			Err(_) => assert!(true),
		}
	}

	#[test]
	fn test_validate_jwt_unexpected_key_algorithm() {
		let helper = Helper::new();

		let kid = "bc63c2e9-5d1c-4e32-9b62-178f60409abd";

		let (modulus, exponent) = helper.get_modulus_and_public_exponent();

		let jwks_key = JwksKey {
			use_key: String::new(),
			kty: String::new(),
			kid: kid.to_string(),
			alg: String::from("ES256"),
			n: modulus,
			e: exponent,
		};
		let jwks = JwksModel { keys: vec![jwks_key] };

		let token = helper.generate_jwt_token(Some(kid), None, false);

		assert!(matches!(validate_jwt(&token, jwks), Err(ClerkError::InternalServerError(_))))
	}

	#[test]
	fn test_validate_jwt_invalid_decoding_key() {
		let helper = Helper::new();

		let kid = "bc63c2e9-5d1c-4e32-9b62-178f60409abd";

		let jwks_key = JwksKey {
			use_key: String::new(),
			kty: String::new(),
			kid: kid.to_string(),
			alg: String::from("RS256"),
			n: String::from("INVALIDMODULUS"),
			e: String::from("INVALIDEXPONENT"),
		};
		let jwks = JwksModel { keys: vec![jwks_key] };

		let token = helper.generate_jwt_token(Some(kid), None, false);

		assert!(matches!(validate_jwt(&token, jwks), Err(ClerkError::InternalServerError(_))))
	}

	#[test]
	fn test_validate_jwt_invalid_jwt_token() {
		let helper = Helper::new();

		let kid = "bc63c2e9-5d1c-4e32-9b62-178f60409abd";

		let jwks_key = JwksKey {
			use_key: String::new(),
			kty: String::new(),
			kid: kid.to_string(),
			alg: String::from("RS256"),
			n: String::new(),
			e: String::new(),
		};
		let jwks = JwksModel { keys: vec![jwks_key] };

		let token = helper.generate_jwt_token(Some(kid), None, false);

		match validate_jwt(token.as_str(), jwks) {
			Ok(_) => unreachable!("Unexpected valid jwt token"),
			Err(_) => assert!(true),
		}
	}

	#[test]
	fn test_token_kid_success() {
		let helper = Helper::new();

		let token = helper.generate_jwt_token(None, None, false);
		let expected = Header::new(Algorithm::RS256);

		match get_token_header(token.as_str()) {
			Ok(header) => assert_eq!(header, expected),
			Err(err) => unreachable!("Unexpected error: {:?}", err),
		}
	}

	#[test]
	fn test_token_kid_error() {
		let token = "invalid_jwt_token";

		match get_token_header(token) {
			Ok(_) => unreachable!("Expected an error, but it succeeded."),
			Err(err) => assert_eq!(err.kind().to_owned(), ErrorKind::InvalidToken, "Unexpected error: {:?}", err),
		}
	}
}
