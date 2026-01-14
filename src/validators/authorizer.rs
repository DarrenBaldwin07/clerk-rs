use crate::{apis::jwks_api::JwksKey, validators::jwks::JwksProvider};
use jsonwebtoken::{decode, decode_header, errors::Error as jwtError, Algorithm, DecodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_aux::prelude::*;
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
pub struct ActiveOrganizationV2 {
	/// The ID of the Organization.
	pub id: String,

	/// The slug of the Organization.
	pub slg: String,

	/// The Role of the user in the Organization, without the org: prefix.
	pub rol: String,

	/// The Permissions of the user in the Organization.
	#[serde(deserialize_with = "deserialize_vec_from_string_or_vec")]
	pub per: Vec<String>,

	/// feature-permission map
	#[serde(deserialize_with = "deserialize_vec_from_string_or_vec")]
	pub fpm: Vec<u16>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Actor {
	pub iss: Option<String>,
	pub sid: Option<String>,
	pub sub: String,
}

/// A union of Clerk JWTs, either v1 or v2.
///
/// Use this type if you need to handle both v1 and v2 JWTs in the same code path.
///
/// This relies on the `v` field as the descriminant for which JWT version is being used.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "v")]
pub enum ClerkJwt {
	#[serde(rename = "2")]
	V2(ClerkJwtV2),
	#[serde(untagged)]
	V1(ClerkJwtV1),
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ClerkJwtV1 {
	pub azp: Option<String>,
	pub exp: i64,
	pub iat: i64,
	pub iss: String,
	pub nbf: i64,
	pub sid: Option<String>,
	pub sub: String,
	pub act: Option<Actor>,
	#[serde(flatten)]
	pub org: Option<ActiveOrganization>,
	/// Catch-all for any other attributes that may be present in the JWT. This
	/// is useful for custom templates that may have additional fields.
	#[serde(flatten)]
	pub other: Map<String, Value>,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct ClerkJwtV2 {
	/// The Origin header that was included in the original Frontend API request made from the user.
	/// Most commonly, it will be the URL of the application. This claim could be omitted if,
	/// for privacy-related reasons, `Origin` is empty or null.
	pub azp: Option<String>,

	/// The time after which the token will expire, as a Unix timestamp.
	/// Determined using the Token lifetime JWT template setting in the [Clerk Dashboard].
	///
	/// [Clerk Dashboard]: https://dashboard.clerk.com/~/jwt-templates
	pub exp: i64,

	/// Each item represents the minutes that have passed since the last time a first factor or
	/// second factor, respectively, was verified.
	pub fva: [i64; 2],

	/// The time at which the token was issued as a Unix timestamp.
	pub iat: i64,

	/// The Frontend API URL of your instance.
	pub iss: String,

	/// The unique identifier of the token.
	pub jti: String,

	/// The time before which the token is considered invalid, as a Unix timestamp.
	/// Determined using the Allowed Clock Skew JWT template setting in the [Clerk Dashboard].
	///
	/// [Clerk Dashboard]: https://dashboard.clerk.com/~/jwt-templates
	pub nbf: i64,

	/// The ID of the current session.
	pub sid: Option<String>,

	/// The ID of the current user of the session.
	pub sub: String,

	/// The Plan that is active. The value is in the format `scope:planslug`, where scope can be `u`
	/// or `o` representing user or Organization-level Plans, respectively.
	///
	/// The `u:` prefix is used if no Organization is active, and the `o:` prefix appears if an
	/// Organization is active.
	pub pla: String,

	/// A list of enabled Features and their scope.
	///
	/// The scope can either be `o` for Organization-level Features if there is an Active
	/// Organization, or `u` for user-level Features or if there is no active Organization.
	#[serde(deserialize_with = "deserialize_vec_from_string_or_vec")]
	pub fea: Vec<String>,

	/// The status of the current session.
	pub sts: Option<String>,

	/// The active Organization claim.
	pub o: Option<ActiveOrganizationV2>,

	/// Catch-all for any other attributes that may be present in the JWT. This
	/// is useful for custom templates that may have additional fields.
	#[serde(flatten)]
	pub other: Map<String, Value>,
}

impl ClerkJwtV2 {
	/// Returns an iterator over the features and their permissions.
	///
	/// The tuple is in the form of `(feature, Iterator<permissions>)`.
	///
	/// If no Organization is active, this will be an empty iterator.
	pub fn permissions_iter(&self) -> impl Iterator<Item = (&str, impl Iterator<Item = &str>)> {
		self.fea
			.iter()
			.zip(self.o.as_ref().map(|org| org.fpm.iter()).unwrap_or_default())
			.map(|(feature, mask)| {
				let perms = self
					.o
					.as_ref()
					.map(|org| org.per.iter())
					.unwrap_or_default()
					.enumerate()
					.filter_map(move |(i, perm)| if mask & (1 << i) != 0 { Some(perm.as_str()) } else { None });

				(feature.as_str(), perms)
			})
	}
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
	pub async fn authorize<T, Jwt>(&self, request: &T) -> Result<Jwt, ClerkError>
	where
		T: ClerkRequest,
		Jwt: DeserializeOwned,
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
pub async fn validate_jwt<J: JwksProvider, Jwt: DeserializeOwned>(token: &str, jwks: Arc<J>) -> Result<Jwt, ClerkError> {
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
pub fn validate_jwt_with_key<Jwt: DeserializeOwned>(token: &str, key: &JwksKey) -> Result<Jwt, ClerkError> {
	match key.alg.as_str() {
		// Currently, clerk only supports Rs256 by default
		"RS256" => {
			let decoding_key = DecodingKey::from_rsa_components(&key.n, &key.e)
				.map_err(|_| ClerkError::InternalServerError(String::from("Error: Invalid decoding key")))?;

			let mut validation = Validation::new(Algorithm::RS256);
			validation.validate_exp = true;
			validation.validate_nbf = true;

			match decode::<Jwt>(token, &decoding_key, &validation) {
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
	struct ClaimsV1 {
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

			let claims = ClaimsV1 {
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
					iss: Some("actor_iss".to_string()),
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

		let expected = ClerkJwtV1 {
			azp: Some("client_id".to_string()),
			sub: "user".to_string(),
			iat: current_time as i64,
			exp: (current_time + 1000) as i64,
			iss: "issuer".to_string(),
			nbf: current_time as i64,
			sid: Some("session_id".to_string()),
			act: Some(Actor {
				iss: Some("actor_iss".to_string()),
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

		assert_eq!(
			validate_jwt_with_key::<ClerkJwtV1>(token.as_str(), &jwks_key).expect("should be valid"),
			expected
		);
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
			validate_jwt_with_key::<ClerkJwtV1>(&token, &jwks_key),
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
			validate_jwt_with_key::<ClerkJwtV1>(&token, &jwks_key),
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

		let res = validate_jwt_with_key::<ClerkJwtV1>(&token, &jwks_key);
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

		let res = validate_jwt_with_key::<ClerkJwtV1>(&token, &jwks_key);
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

		let expected = ClerkJwtV1 {
			azp: Some("client_id".to_string()),
			sub: "user".to_string(),
			iat: current_time as i64,
			exp: (current_time + 1000) as i64,
			iss: "issuer".to_string(),
			nbf: current_time as i64,
			sid: Some("session_id".to_string()),
			act: Some(Actor {
				iss: Some("actor_iss".to_string()),
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

		assert_eq!(
			validate_jwt::<_, ClerkJwtV1>(token.as_str(), jwks).await.expect("should be valid"),
			expected
		);
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

		assert!(matches!(
			validate_jwt::<_, ClerkJwtV1>("invalid_token", jwks).await,
			Err(ClerkError::Unauthorized(_))
		))
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

		assert!(matches!(
			validate_jwt::<_, ClerkJwtV1>(&token, jwks).await,
			Err(ClerkError::Unauthorized(_))
		))
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

		assert!(matches!(
			validate_jwt::<_, ClerkJwtV1>(&token, jwks).await,
			Err(ClerkError::Unauthorized(_))
		))
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
