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
	exclude_routes: Option<Vec<String>>,
}

impl<J: JwksProvider> ClerkPoemMiddleware<J> {
	pub fn new(jwks_provider: J, validate_session_cookie: bool, exclude_routes: Option<Vec<String>>) -> Self {
		let authorizer = ClerkAuthorizer::new(jwks_provider, validate_session_cookie);
		Self { authorizer, exclude_routes }
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
			exclude_routes: self.exclude_routes.clone(),
			ep,
		}
	}
}

/// A middleware to authenticate Poem routes using Clerk.com.
///
/// If auth succeeds, the JWT data is available using `Data<&ClerkJwt>` (or
/// `req.data::<ClerkJwt>()`).
pub struct ClerkPoemMiddlewareImpl<J, E> {
	authorizer: ClerkAuthorizer<J>,
	exclude_routes: Option<Vec<String>>,
	ep: E,
}

impl<E: Endpoint, J: JwksProvider + Send + Sync> Endpoint for ClerkPoemMiddlewareImpl<J, E>
where
	E: poem::Endpoint<Output = Response>,
{
	type Output = Response;

	#[cfg(feature="tracing")]
	#[tracing::instrument(
		skip_all, 
		name="clerk_poem_middleware", 
		fields(
			req.remote_addr = req.remote_addr().to_string(), 
			req.uri = req.uri().to_string(),
			req.method = req.method().to_string()
		)
	)]
	async fn call(&self, mut req: Request) -> Result<Self::Output> {
		
		#[cfg(feature="tracing")]
		tracing::trace!("Auth middleware entered");
		
		if let Some(exclude_routes) = &self.exclude_routes {
			if exclude_routes.iter().any(|r| r == req.uri().path()) {
				
				#[cfg(feature="tracing")]
				tracing::info!("Route excluded from auth, skipping auth.");
				
				#[cfg(feature="tracing")]
				tracing::trace!("Auth middleware exited");

				// call next and early return
				return self.ep.call(req).await;
			}
		}

		match self.authorizer.authorize(&req).await {
			Ok(jwt) => {
				
				#[cfg(feature="tracing")]
				tracing::info!("Authed request; user is: {}", &jwt.sub);
				
				#[cfg(feature="tracing")]
				tracing::trace!("Auth middleware exited");

				// This can be accessed using Data<&ClerkJwt>
				req.set_data(jwt);

				// call next
				self.ep.call(req).await
			}
			Err(error) => match &error {
				// The error strings are passed through with the correct status code
				ClerkError::Unauthorized(msg) => {
					
					#[cfg(feature="tracing")]
					tracing::info!("Middleware blocked unauthorized: {}", &msg);
					
					#[cfg(feature="tracing")]
					tracing::trace!("Auth middleware exited");

					Err(Unauthorized(error))
				},
				ClerkError::InternalServerError(msg) => {
					
					#[cfg(feature="tracing")]
					tracing::error!("Internal Server Error with auth middleware: {}", &msg);

					#[cfg(feature="tracing")]
					tracing::trace!("Auth middleware exited");
					
					Err(InternalServerError(error))
				},
			},
		}
	}
}
