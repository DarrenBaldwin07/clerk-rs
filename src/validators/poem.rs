use poem::{
	error::{InternalServerError, Unauthorized},
	Endpoint, Middleware, Request, Response, Result,
};

use super::{
	authorizer::{ClerkAuthorizer, ClerkError, ClerkRequest},
	jwks::JwksProvider,
};

impl ClerkRequest for Request {
	fn get_header(&self, key: &str) -> Option<String> {
		self.header(key).map(|s| s.to_string())
	}

	fn get_cookie(&self, key: &str) -> Option<String> {
		let jar = self.cookie();
		jar.get(key).map(|c| c.value_str().to_string())
	}
}

// The below implementation is derived from: https://docs.rs/poem/latest/poem/middleware/trait.Middleware.html#create-your-own-middleware

pub struct ClerkPoemMiddleware<J> {
	authorizer: ClerkAuthorizer<J>,
}

impl<J: JwksProvider> ClerkPoemMiddleware<J> {
	pub fn new(jwks_provider: J, validate_session_cookie: bool) -> Self {
		let authorizer = ClerkAuthorizer::new(jwks_provider, validate_session_cookie);
		Self { authorizer }
	}
}

impl<E: Endpoint, J: JwksProvider + Send + Sync> Middleware<E> for ClerkPoemMiddleware<J>
where
	E: poem::Endpoint<Output = Response>,
{
	type Output = ClerkPoemMiddlewareImpl<J, E>;

	fn transform(&self, ep: E) -> Self::Output {
		Self::Output {
			authorizer: self.authorizer.clone(),
			ep,
		}
	}
}

pub struct ClerkPoemMiddlewareImpl<J, E> {
	authorizer: ClerkAuthorizer<J>,
	ep: E,
}

impl<E: Endpoint, J: JwksProvider + Send + Sync> Endpoint for ClerkPoemMiddlewareImpl<J, E>
where
	E: poem::Endpoint<Output = Response>,
{
	type Output = Response;

	async fn call(&self, mut req: Request) -> Result<Self::Output> {
		match self.authorizer.authorize(&req).await {
			Ok(jwt) => {
				// This can be accessed using Data<&ClerkJwt>
				req.set_data(jwt);

				// call next
				self.ep.call(req).await
			}
			Err(error) => match error {
				// The error strings are passed through with the correct status code
				ClerkError::Unauthorized(_) => Err(Unauthorized(error)),
				ClerkError::InternalServerError(_) => Err(InternalServerError(error)),
			},
		}
	}
}
