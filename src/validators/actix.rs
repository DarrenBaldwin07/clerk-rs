use crate::{
	apis::jwks_api::{Jwks, JwksModel},
	clerk::Clerk,
	ClerkConfiguration,
};
use actix_web::{
	body::EitherBody,
	dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
	error::Error,
	http::header::HeaderValue,
	HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, decode_header, errors::Error as jwtError, Algorithm, DecodingKey, Header, Validation};
use std::{
	future::{ready, Ready},
	rc::Rc,
};

#[derive(Debug, Serialize, Deserialize)]
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

/// Validate a jwt using a jwks set
pub fn validate_jwt(token: &str, jwks: JwksModel) -> Result<(bool, ClerkJwt), bool> {
	let parsed_jwt = token.replace("Bearer ", "");
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
			// Currently, clerk only supports Rs256 by default (could be wrong here -- might need to look into this)
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
			_ => unreachable!("this should be a RSA"),
		}
	// In the event that a matching jwk was not found we want to output a false result (showing that the jwt was invalid)
	} else {
		return Err(false);
	}
}

/// Authorize a actix-web route given a `clerk_client` and a valid service request to an actix-web endpoint
pub async fn clerk_authorize(req: &ServiceRequest, clerk_client: &Clerk) -> Result<(bool, ClerkJwt), HttpResponse> {
	// Get our jwks from Clerk.dev
	let jwks = match Jwks::get_jwks(clerk_client).await {
		Ok(val) => val,
		Err(_) => return Err(HttpResponse::InternalServerError().json("Error: Could not fetch JWKS!")),
	};
	// Parse the request headers
	let access_token: &str = match req.headers().get("Authorization") {
		Some(val) => match val.to_str() {
			Ok(val) => val,
			Err(_) => return Err(HttpResponse::Unauthorized().json("Error: Unable to parse http header")),
		},
		None => return Err(HttpResponse::Unauthorized().json("Error: No Authorization header found on the request payload!")),
	};

	// Finally, check if the jwt is valid...
	match validate_jwt(access_token, jwks) {
		Ok(val) => Ok(val),
		Err(_) => return Err(HttpResponse::Unauthorized().json("Error: Invalid JWT!")),
	}
}

pub fn parse_cookies(req: &ServiceRequest) -> Option<&HeaderValue> {
	req.headers().get("cookie")
}

/// Actix-web middleware for protecting a http endpoint with Cerk.dev
pub struct ClerkMiddleware {
	pub clerk_config: ClerkConfiguration,
	pub routes: Option<Vec<String>>
}

impl ClerkMiddleware {
	pub fn new(config: ClerkConfiguration, routes: Option<Vec<String>>) -> Self {
		Self { clerk_config: config, routes }
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
			routes: self.routes.clone()
		}))
	}
}

pub struct ClerkMiddlewareService<S> {
	service: Rc<S>,
	config: ClerkConfiguration,
	routes: Option<Vec<String>>
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

		// We want to skip running the validator if we are not able to find a matching path from the listed valid paths provided by the user
		match self.routes.clone() {
			Some(route_matches) => {
				// If the user only wants to apply authentication to a select amount of routes, we handle that logic here
				let path = req.path();
				// Check if the path was NOT contained inside of the routes specified by the user...
				if !route_matches.contains(&path.to_owned()) {
					// Since the path was not inside of the listed routes we want to trigger an early exit
					return Box::pin(async move {
						let res = svc.call(req).await?;
						return Ok(res.map_into_left_body());
					});
				}
			},
			// Since we did find a matching route we can simply do nothing here and start the actual auth logic...
			None => {}
		}

		Box::pin(async move {
			// Check if the request is authenticated
			let is_authed = clerk_authorize(&req, &client).await;

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
