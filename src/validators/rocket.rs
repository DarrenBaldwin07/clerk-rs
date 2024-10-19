use crate::validators::{
	authorizer::{ClerkAuthorizer, ClerkError, ClerkRequest},
	jwks::JwksProvider,
};
use rocket::{
	http::Status,
	request::{FromRequest, Outcome},
	Request,
};

use super::authorizer::ClerkJwt;

// Implement ClerkRequest for Rocket's Request
impl<'r> ClerkRequest for &'r Request<'_> {
	fn get_header(&self, key: &str) -> Option<String> {
		self.headers().get_one(key).map(|s| s.to_string())
	}

	fn get_cookie(&self, key: &str) -> Option<String> {
		self.cookies().get(key).map(|cookie| cookie.value().to_string())
	}
}

pub struct ClerkGuardConfig<J: JwksProvider> {
	pub authorizer: ClerkAuthorizer<J>,
	pub routes: Option<Vec<String>>,
}

impl<J: JwksProvider> ClerkGuardConfig<J> {
	pub fn new(jwks_provider: J, routes: Option<Vec<String>>, validate_session_cookie: bool) -> Self {
		let authorizer = ClerkAuthorizer::new(jwks_provider, validate_session_cookie);
		Self { authorizer, routes }
	}
}

pub struct ClerkGuard<J: JwksProvider + Send + Sync> {
	pub jwt: Option<ClerkJwt>,
	_marker: std::marker::PhantomData<J>,
}

// Implement request guard for ClerkGuard
#[rocket::async_trait]
impl<'r, J: JwksProvider + Send + Sync + 'static> FromRequest<'r> for ClerkGuard<J> {
	type Error = ClerkError;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		// Retrieve the ClerkAuthorizer from managed state
		let config = request
			.rocket()
			.state::<ClerkGuardConfig<J>>()
			.expect("ClerkGuardConfig not found in managed state");

		match &config.routes {
			Some(route_matches) => {
				// If the user only wants to apply authentication to a select amount of routes, we handle that logic here
				let path = request.uri().path();
				// Check if the path was NOT contained inside of the routes specified by the user...
				let path_not_in_specified_routes = route_matches.iter().find(|&route| route == &path.to_string()).is_none();

				if path_not_in_specified_routes {
					// Since the path was not inside of the listed routes we want to trigger an early exit
					return Outcome::Success(ClerkGuard {
						jwt: None,
						_marker: std::marker::PhantomData,
					});
				}
			}
			// Since we did find a matching route we can simply do nothing here and start the actual auth logic...
			None => {}
		}

		match config.authorizer.authorize(&request).await {
			Ok(jwt) => {
				request.local_cache(|| jwt.clone());
				return Outcome::Success(ClerkGuard {
					jwt: Some(jwt),
					_marker: std::marker::PhantomData,
				});
			}
			Err(error) => match error {
				ClerkError::Unauthorized(msg) => Outcome::Error((Status::Unauthorized, ClerkError::Unauthorized(msg))),
				ClerkError::InternalServerError(msg) => Outcome::Error((Status::InternalServerError, ClerkError::InternalServerError(msg))),
			},
		}
	}
}
