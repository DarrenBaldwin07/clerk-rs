use crate::{apis::jwks_api::JwksKey, validators::jwks::JwksProvider};
use jsonwebtoken::{decode, decode_header, errors::Error as jwtError, Algorithm, DecodingKey, Header, Validation};
use serde_json::{Map, Value};
use std::{error::Error, fmt, sync::Arc};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ActiveOrganization {
	#[serde(rename = "org_id")]
	pub id: String,
	#[serde(rename = "org_slug")]
	pub slug: String,
	#[serde(rename = "org_role")]
	pub role: String,
	#[serde(rename = "org_permissions")]
	pub permissions: Vec<String>,
}

impl ActiveOrganization {
	/// Checks if the user has the specific permission in their session claims.
	pub fn has_permission(&self, permission: &str) -> bool {
		self.permissions.contains(&permission.to_string())
	}

	/// Checks if the user has the specific role in their session claims.
	/// Performing role checks is not considered a best-practice and developers
	/// should avoid it as much as possible. Usually, complex role checks can be
	/// refactored with a single permission check.
	pub fn has_role(&self, role: &str) -> bool {
		self.role == role
	}
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Actor {
	pub iss: String,
	pub sid: Option<String>,
	pub sub: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ClerkJwt {
	pub azp: Option<String>,
	pub exp: i32,
	pub iat: i32,
	pub iss: String,
	pub nbf: i32,
	pub sid: Option<String>,
	pub sub: String,
	pub act: Option<Actor>,
	#[serde(flatten)]
	pub org: Option<ActiveOrganization>,
	/// Catch-all for any other attributes that may be present in the JWT. This
	/// is useful for custom templates that may have additional fields
	#[serde(flatten)]
	pub other: Map<String, Value>,
}

pub trait ClerkRequest {
	fn get_header(&self, key: &str) -> Option<String>;
	fn get_cookie(&self, key: &str) -> Option<String>;
}

#[derive(Clone, Debug)]
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

pub struct ClerkAuthorizer<J> {
	jwks_provider: Arc<J>,
	validate_session_cookie: bool,
}

impl<J: JwksProvider> ClerkAuthorizer<J> {
	/// Creates a Clerk authorizer
	pub fn new(jwks_provider: J, validate_session_cookie: bool) -> Self {
		Self {
			jwks_provider: Arc::new(jwks_provider),
			validate_session_cookie,
		}
	}

	/// Returns a reference to the underlying [`JwksProvider`].
	pub fn jwks_provider(&self) -> &Arc<J> {
		&self.jwks_provider
	}

	/// Authorizes a service request against the Clerk auth provider
	pub async fn authorize<T>(&self, request: &T) -> Result<ClerkJwt, ClerkError>
	where
		T: ClerkRequest,
	{
		// get the jwt from header or cookies
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

		validate_jwt(&access_token, self.jwks_provider.clone()).await
	}
}

impl<J> Clone for ClerkAuthorizer<J> {
	fn clone(&self) -> Self {
		Self {
			jwks_provider: self.jwks_provider.clone(),
			validate_session_cookie: self.validate_session_cookie,
		}
	}
}

/// Validates a jwt using the given [`JwksProvider`].
///
/// The jwt is required to have a `kid` which is used to request the matching key from the provider.
pub async fn validate_jwt<J: JwksProvider>(token: &str, jwks: Arc<J>) -> Result<ClerkJwt, ClerkError> {
	// parse the header to get the kid
	let kid = match get_token_header(token).map(|h| h.kid) {
		Ok(Some(kid)) => kid,
		_ => {
			// if the kid header was invalid or the kid field was unset, error
			return Err(ClerkError::Unauthorized(String::from("Error: Invalid JWT!")));
		}
	};

	// get the key from the provider
	let Ok(key) = jwks.get_key(&kid).await else {
		// In the event that a matching jwk was not found we want to output an error
		return Err(ClerkError::Unauthorized(String::from("Error: Invalid JWT!")));
	};

	validate_jwt_with_key(token, &key)
}

/// Validates a jwt using the given jwk.
///
/// This function does not check that the token's kid matches the key's.
pub fn validate_jwt_with_key(token: &str, key: &JwksKey) -> Result<ClerkJwt, ClerkError> {
	match key.alg.as_str() {
		// Currently, clerk only supports Rs256 by default
		"RS256" => {
			let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e)
				.map_err(|_| ClerkError::InternalServerError(String::from("Error: Invalid decoding key")))?;

			let mut validation = Validation::new(Algorithm::RS256);
			validation.validate_exp = true;
			validation.validate_nbf = true;

			match decode::<ClerkJwt>(token, &decoding_key, &validation) {
				Ok(token) => Ok(token.claims),
				Err(err) => Err(ClerkError::Unauthorized(format!("Error: Invalid JWT! cause: {}", err))),
			}
		}
		_ => Err(ClerkError::InternalServerError(String::from("Error: Unsupported key algorithm"))),
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
	use crate::{apis::jwks_api::JwksKey, validators::jwks::tests::StaticJwksProvider};
	use base64::engine::general_purpose::URL_SAFE_NO_PAD;
	use base64::prelude::*;
	use jsonwebtoken::{encode, errors::ErrorKind, Algorithm, EncodingKey, Header};
	use rsa::{pkcs1::EncodeRsaPrivateKey, traits::PublicKeyParts, RsaPrivateKey};
	use std::time::{SystemTime, UNIX_EPOCH};

	#[derive(Debug, serde::Serialize)]
	struct CustomFields {
		custom_attribute: String,
	}

	#[derive(Debug, serde::Serialize)]
	struct Claims {
		sub: String,
		iat: usize,
		nbf: usize,
		exp: usize,
		azp: String,
		iss: String,
		sid: String,
		act: Actor,
		org_id: String,
		org_slug: String,
		org_role: String,
		org_permissions: Vec<String>,
		custom_key: String,
		custom_map: CustomFields,
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

			let mut current_time = current_time.unwrap_or(SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize);

			if expired {
				// issue the token some time in the past so that it's expired now
				current_time -= 5000;
			}

			// expire after 1000 secs
			let expiration = current_time + 1000;

			let claims = Claims {
				azp: "client_id".to_string(),
				sub: "user".to_string(),
				iat: current_time,
				exp: expiration,
				iss: "issuer".to_string(),
				nbf: current_time,
				sid: "session_id".to_string(),
				org_id: "org_id".to_string(),
				org_slug: "org_slug".to_string(),
				org_role: "org_role".to_string(),
				org_permissions: vec!["org_permission".to_string()],
				act: Actor {
					iss: "actor_iss".to_string(),
					sid: Some("actor_sid".to_string()),
					sub: "actor_sub".to_string(),
				},
				custom_key: "custom_value".to_string(),
				custom_map: CustomFields {
					custom_attribute: "custom_attribute".to_string(),
				},
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
	fn test_validate_jwt_with_key_success() {
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

		let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
		let token = helper.generate_jwt_token(Some(kid), Some(current_time), false);

		let expected = ClerkJwt {
			azp: Some("client_id".to_string()),
			sub: "user".to_string(),
			iat: current_time as i32,
			exp: (current_time + 1000) as i32,
			iss: "issuer".to_string(),
			nbf: current_time as i32,
			sid: Some("session_id".to_string()),
			act: Some(Actor {
				iss: "actor_iss".to_string(),
				sid: Some("actor_sid".to_string()),
				sub: "actor_sub".to_string(),
			}),
			org: Some(ActiveOrganization {
				id: "org_id".to_string(),
				slug: "org_slug".to_string(),
				role: "org_role".to_string(),
				permissions: vec!["org_permission".to_string()],
			}),
			other: {
				let mut map = Map::new();
				map.insert("custom_key".to_string(), Value::String("custom_value".to_string()));
				map.insert(
					"custom_map".to_string(),
					Value::Object({
						let mut map = Map::new();
						map.insert("custom_attribute".to_string(), Value::String("custom_attribute".to_string()));
						map
					}),
				);
				map
			},
		};

		assert_eq!(validate_jwt_with_key(token.as_str(), &jwks_key).expect("should be valid"), expected);
	}

	#[test]
	fn test_validate_jwt_with_key_unexpected_key_algorithm() {
		let helper = Helper::new();

		let kid = "bc63c2e9-5d1c-4e32-9b62-178f60409abd";

		let (modulus, exponent) = helper.get_modulus_and_public_exponent();

		let jwks_key = JwksKey {
			use_key: String::new(),
			kty: String::new(),
			kid: kid.to_string(),
			alg: String::from("INVALIDALGORITHM"),
			n: modulus,
			e: exponent,
		};

		let token = helper.generate_jwt_token(Some(kid), None, false);

		assert!(matches!(
			validate_jwt_with_key(&token, &jwks_key),
			Err(ClerkError::InternalServerError(_))
		))
	}

	#[test]
	fn test_validate_jwt_with_key_invalid_decoding_key() {
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

		let token = helper.generate_jwt_token(Some(kid), None, false);

		assert!(matches!(
			validate_jwt_with_key(&token, &jwks_key),
			Err(ClerkError::InternalServerError(_))
		))
	}

	#[test]
	fn test_validate_jwt_with_key_invalid_sig() {
		let helper1 = Helper::new();
		let helper2 = Helper::new();

		let kid = "bc63c2e9-5d1c-4e32-9b62-178f60409abd";

		let (modulus, exponent) = helper1.get_modulus_and_public_exponent();

		let jwks_key = JwksKey {
			use_key: String::new(),
			kty: String::new(),
			kid: kid.to_string(),
			alg: String::from("RS256"),
			n: modulus,
			e: exponent,
		};

		let token = helper2.generate_jwt_token(None, None, false);

		let res = validate_jwt_with_key(&token, &jwks_key);
		assert!(matches!(res, Err(ClerkError::Unauthorized(_))));
	}

	#[test]
	fn test_validate_jwt_with_key_expired() {
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

		let token = helper.generate_jwt_token(Some(kid), None, true);

		let res = validate_jwt_with_key(&token, &jwks_key);
		assert!(matches!(res, Err(ClerkError::Unauthorized(_))))
	}

	#[tokio::test]
	async fn test_validate_jwt_success() {
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
		let jwks = Arc::new(StaticJwksProvider::from_key(jwks_key));

		let current_time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs() as usize;
		let token = helper.generate_jwt_token(Some(kid), Some(current_time), false);

		let expected = ClerkJwt {
			azp: Some("client_id".to_string()),
			sub: "user".to_string(),
			iat: current_time as i32,
			exp: (current_time + 1000) as i32,
			iss: "issuer".to_string(),
			nbf: current_time as i32,
			sid: Some("session_id".to_string()),
			act: Some(Actor {
				iss: "actor_iss".to_string(),
				sid: Some("actor_sid".to_string()),
				sub: "actor_sub".to_string(),
			}),
			org: Some(ActiveOrganization {
				id: "org_id".to_string(),
				slug: "org_slug".to_string(),
				role: "org_role".to_string(),
				permissions: vec!["org_permission".to_string()],
			}),
			other: {
				let mut map = Map::new();
				map.insert("custom_key".to_string(), Value::String("custom_value".to_string()));
				map.insert(
					"custom_map".to_string(),
					Value::Object({
						let mut map = Map::new();
						map.insert("custom_attribute".to_string(), Value::String("custom_attribute".to_string()));
						map
					}),
				);
				map
			},
		};

		assert_eq!(validate_jwt(token.as_str(), jwks).await.expect("should be valid"), expected);
	}

	#[tokio::test]
	async fn test_validate_jwt_invalid_token() {
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
		let jwks = Arc::new(StaticJwksProvider::from_key(jwks_key));

		assert!(matches!(validate_jwt("invalid_token", jwks).await, Err(ClerkError::Unauthorized(_))))
	}

	#[tokio::test]
	async fn test_validate_jwt_missing_kid() {
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
		let jwks = Arc::new(StaticJwksProvider::from_key(jwks_key));

		let token = helper.generate_jwt_token(None, None, false);

		assert!(matches!(validate_jwt(&token, jwks).await, Err(ClerkError::Unauthorized(_))))
	}

	#[tokio::test]
	async fn test_validate_jwt_unknown_key() {
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
		let jwks = Arc::new(StaticJwksProvider::from_key(jwks_key));

		let token = helper.generate_jwt_token(Some("bc63c2e9-5d1c-4e32-9b62-178f60409abd"), None, false);

		assert!(matches!(validate_jwt(&token, jwks).await, Err(ClerkError::Unauthorized(_))))
	}

	#[test]
	fn test_helper_generate_token_header() {
		let helper = Helper::new();

		let token = helper.generate_jwt_token(None, None, false);
		let expected = Header::new(Algorithm::RS256);

		assert_eq!(get_token_header(&token).expect("should be valid"), expected);
	}

	#[test]
	fn test_helper_generate_token_header_with_kid() {
		let helper = Helper::new();

		let kid = "bc63c2e9-5d1c-4e32-9b62-178f60409abd".to_string();

		let token = helper.generate_jwt_token(Some(&kid), None, false);
		let mut expected = Header::new(Algorithm::RS256);
		expected.kid = Some(kid);

		assert_eq!(get_token_header(&token).expect("should be valid"), expected);
	}

	#[test]
	fn test_helper_generate_token_header_error() {
		let token = "invalid_jwt_token";

		let err = get_token_header(token).expect_err("should be invalid");
		assert_eq!(err.kind().to_owned(), ErrorKind::InvalidToken);
	}
}
