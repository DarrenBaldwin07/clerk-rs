use crate::{
	apis::jwks_api::{Jwks, JwksModel},
	clerk::Clerk,
	ClerkConfiguration,
};
use actix_web::{
	body::EitherBody,
	dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
	error::Error,
	HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, decode_header, errors::Error as jwtError, Algorithm, DecodingKey, Header, Validation};
use std::{
	future::{ready, Ready},
	rc::Rc,
};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ClerkJwt {
	pub azp: String,
	pub exp: i32,
	pub iat: i32,
	pub iss: String,
	pub nbf: i32,
	pub sid: String,
	pub sub: String,
}

/// Extract token kid from a jwt passed as header to an actix-web endpoint
pub fn token_kid(jwt: String) -> Result<Header, jwtError> {
	let header = decode_header(&jwt);
	header
}

/// Validate a jwt using a jwks
pub fn validate_jwt(token: &str, jwks: JwksModel) -> Result<(bool, ClerkJwt), bool> {
	let parsed_jwt = token;
	// If we were not able to parse the kid field we want to output an invalid case...
	let kid = match token_kid(parsed_jwt.to_owned()) {
		Ok(val) => val.kid,
		Err(_) => {
			return Err(false);
		}
	};

	let jwk = jwks.keys.iter().find(|k| &k.kid == kid.as_ref().unwrap());

	// Check to see if we found a valid jwk key with the token kid
	if let Some(j) = jwk {
		match j.alg.as_str() {
			// Currently, clerk only supports Rs256 by default
			"RS256" => {
				let decoding_key = DecodingKey::from_rsa_components(&j.n, &j.e).unwrap();
				let mut validation = Validation::new(Algorithm::RS256);
				validation.validate_exp = true;
				let decoded_token = decode::<ClerkJwt>(&parsed_jwt, &decoding_key, &validation);

				return match decoded_token {
					Ok(token) => Ok((true, token.claims)),
					_ => Err(false),
				};
			}
			_ => unreachable!("This should be a RSA"),
		}
	// In the event that a matching jwk was not found we want to output a false result (showing that the jwt was invalid)
	} else {
		return Err(false);
	}
}

// This is public and exported for crate users to consume but doing this is not recommended
/// Authorize a actix-web route given a `clerk_client` and a valid service request to an actix-web endpoint
pub async fn clerk_authorize(req: &ServiceRequest, clerk_client: &Clerk, validate_session_cookie: bool) -> Result<(bool, ClerkJwt), HttpResponse> {
	// Get our jwks from Clerk.dev
	let jwks = match Jwks::get_jwks(clerk_client).await {
		Ok(val) => val,
		Err(_) => return Err(HttpResponse::InternalServerError().json("Error: Could not fetch JWKS!")),
	};
	// Parse the request headers and prefer the `Authorization` header over the session cookie if provided
	let access_token: String = match req.headers().get("Authorization") {
		Some(val) => match val.to_str() {
			Ok(val) => val.to_string().replace("Bearer ", ""),
			Err(_) => return Err(HttpResponse::Unauthorized().json("Error: Unable to parse http header")),
		},
		None => match validate_session_cookie {
			true => match req.cookie("__session") {
				Some(cookie) => cookie.value().to_string(),
				None => {
					return Err(HttpResponse::Unauthorized().json("Error: No Authorization header or session cookie found on the request payload!"))
				}
			},
			false => return Err(HttpResponse::Unauthorized().json("Error: No Authorization header found on the request payload!")),
		},
	};

	// Finally, check if the jwt is valid...
	match validate_jwt(&access_token, jwks) {
		Ok(val) => Ok(val),
		Err(_) => return Err(HttpResponse::Unauthorized().json("Error: Invalid JWT!")),
	}
}

/// Actix-web middleware for protecting a http endpoint with Clerk.dev
/// # Example
/// ```
/// async fn index() -> impl Responder {
/// 	"Hello world!"
/// }
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
/// 	HttpServer::new(|| {
/// 		let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
/// 		App::new().service(
/// 			// prefixes all resources and routes attached to it with /app...
/// 			web::scope("/app")
/// 				.wrap(ClerkMiddleware::new(config, None))
/// 				// ...this route is then protected by Clerk!
/// 				.route("/index", web::get().to(index)),
/// 		)
/// 	})
/// 	.bind(("127.0.0.1", 8080))?
/// 	.run()
/// 	.await
/// }
/// ```
pub struct ClerkMiddleware {
	pub clerk_config: ClerkConfiguration,
	pub routes: Option<Vec<String>>,
	pub should_validate_session_cookie: bool,
}

impl ClerkMiddleware {
	pub fn new(config: ClerkConfiguration, routes: Option<Vec<String>>, should_validate_session_cookie: bool) -> Self {
		Self {
			clerk_config: config,
			routes,
			should_validate_session_cookie,
		}
	}
}

impl<S: 'static, B> Transform<S, ServiceRequest> for ClerkMiddleware
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<EitherBody<B>>;
	type Error = Error;
	type InitError = ();
	type Transform = ClerkMiddlewareService<S>;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ready(Ok(ClerkMiddlewareService {
			service: Rc::new(service),
			config: self.clerk_config.clone(),
			routes: self.routes.clone(),
			should_validate_session_cookie: self.should_validate_session_cookie,
		}))
	}
}

pub struct ClerkMiddlewareService<S> {
	service: Rc<S>,
	config: ClerkConfiguration,
	routes: Option<Vec<String>>,
	should_validate_session_cookie: bool,
}

impl<S: 'static, B> Service<ServiceRequest> for ClerkMiddlewareService<S>
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
{
	type Response = ServiceResponse<EitherBody<B>>;
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

	forward_ready!(service);

	fn call(&self, req: ServiceRequest) -> Self::Future {
		// Initialize our Clerk client with the newly created configuration
		let client = Clerk::new(self.config.clone());

		let svc = self.service.clone();
		let test = self.should_validate_session_cookie.clone();

		// We want to skip running the validator if we are not able to find a matching path from the listed valid paths provided by the user
		match &self.routes {
			Some(route_matches) => {
				// If the user only wants to apply authentication to a select amount of routes, we handle that logic here
				let path = req.path();
				// Check if the path was NOT contained inside of the routes specified by the user...
				let path_not_in_specified_routes = route_matches.iter().find(|&route| route == path).is_none();

				if path_not_in_specified_routes {
					// Since the path was not inside of the listed routes we want to trigger an early exit
					return Box::pin(async move {
						let res = svc.call(req).await?;
						return Ok(res.map_into_left_body());
					});
				}
			}
			// Since we did find a matching route we can simply do nothing here and start the actual auth logic...
			None => {}
		}

		Box::pin(async move {
			// Check if the request is authenticated
			let is_authed = clerk_authorize(&req, &client, test).await;

			match is_authed {
				// If we got a boolean response then lets check if it was either true or false
				Ok(val) => match val.0 {
					// If it was true then we have authed request and can pass the user onto the next body
					true => {
						let res = svc.call(req).await?;
						return Ok(res.map_into_left_body());
					}
					// If it was false we want to throw an unauthed error
					false => {
						return Ok(ServiceResponse::new(
							req.into_parts().0,
							HttpResponse::Unauthorized()
								.body("Unauthorized. All requests must contain a valid Clerk jwt.")
								.map_into_right_body(),
						));
					}
				},
				// Output any other errors thrown fromn the clerk_authorize function
				Err(e) => {
					return Ok(ServiceResponse::new(req.into_parts().0, e.map_into_right_body()));
				}
			}
		})
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::apis::jwks_api::JwksKey;
	use crate::ClerkConfiguration;
	use actix_web::http::header::HeaderValue;
	use actix_web::test as actix_test;

	#[allow(dead_code)]
	async fn test_clerk_authorize() {
		let clerk_config = ClerkConfiguration::new(None, None, Some("YOUR_CLERK_SECRET_KEY".to_string()), None);
		let client = Clerk::new(clerk_config);
		let req = actix_test::TestRequest::default()
			.append_header((actix_web::http::header::AUTHORIZATION, HeaderValue::from_static("<your-api-key>")))
			.to_srv_request();

		let result = clerk_authorize(&req, &client, false).await;

		assert!(result.is_ok())
	}

	#[test]
	fn test_validate_jwt() {
		// Create a sample invalid JWT and a corresponding JwksModel
		let invalid_jwt = "Bearer invalid_token";
		let jwks = JwksModel {
			keys: vec![JwksKey {
				use_key: "sig".to_string(),
				kty: "RSA".to_string(),
				kid: "valid_kid".to_string(),
				alg: "RS256".to_string(),
				n: "valid_n".to_string(),
				e: "valid_e".to_string(),
			}],
		};

		// Call the validate_jwt function with the invalid token and JwksModel
		let result = validate_jwt(invalid_jwt, jwks);

		assert_eq!(result, Err(false));
	}
}
