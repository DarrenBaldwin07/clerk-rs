use crate::validators::{
	authorizer::{ClerkAuthorizer, ClerkError, ClerkRequest},
	jwks::JwksProvider,
};
use rocket::{
	http::Status,
	request::{FromRequest, Outcome},
	Request,
};

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
	_unused: std::marker::PhantomData<J>,
}

// Implement request guard for ClerkGuard
#[rocket::async_trait]
impl<'r, J: JwksProvider + Send + Sync + 'static> FromRequest<'r> for ClerkGuard<J> {
	type Error = ClerkError;

	async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
		// Retrieve the ClerkAuthorizer from managed state
		let authorizer = request
			.rocket()
			.state::<ClerkGuardConfig<J>>()
			.expect("ClerkAuthorizer not found in managed state");

		match authorizer.authorizer.authorize(&request).await {
			Ok(_) => Outcome::Success(ClerkGuard {
				_unused: std::marker::PhantomData,
			}),
			Err(error) => match error {
				ClerkError::Unauthorized(msg) => Outcome::Error((Status::Unauthorized, ClerkError::Unauthorized(msg))),
				ClerkError::InternalServerError(msg) => Outcome::Error((Status::InternalServerError, ClerkError::InternalServerError(msg))),
			},
		}
	}
}
