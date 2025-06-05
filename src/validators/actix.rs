use crate::validators::{
	authorizer::{ClerkAuthorizer, ClerkError, ClerkRequest},
	jwks::JwksProvider,
};
use actix_web::{
	body::EitherBody,
	dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
	error::Error,
	HttpMessage, HttpResponse,
};
use futures_util::future::LocalBoxFuture;
use std::{
	future::{ready, Ready},
	rc::Rc,
};

impl ClerkRequest for ServiceRequest {
	fn get_header(&self, key: &str) -> Option<String> {
		match self.headers().get(key) {
			Some(header) => Some(header.to_str().expect("header must be a string").to_string()),
			None => None,
		}
	}

	fn get_cookie(&self, key: &str) -> Option<String> {
		match self.cookie(key) {
			Some(cookie) => Some(cookie.value().to_string()),
			None => None,
		}
	}
}

/// Actix-web middleware for protecting a http endpoint with Clerk.dev.
///
/// # Example
/// ```
/// async fn index() -> impl Responder {
///     "Hello world!"
/// }
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     HttpServer::new(|| {
///         // Get secret key from environment variable
///         let secret_key = std::env::var("CLERK_SECRET_KEY").unwrap_or_else(|_| {
///             eprintln!("Warning: CLERK_SECRET_KEY environment variable not set");
///             String::new()
///         });
///         
///         let config = ClerkConfiguration::new(None, None, if !secret_key.is_empty() { Some(secret_key) } else { None }, None);
///         let clerk = Clerk::new(config);
///
///         App::new()
///             .wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
///             .route("/index", web::get().to(index))
///     })
///     .bind(("127.0.0.1", 8080))?
///     .run()
///     .await
/// }
/// ```
pub struct ClerkMiddleware<J> {
	pub authorizer: ClerkAuthorizer<J>,
	pub routes: Option<Vec<String>>,
}

impl<J: JwksProvider> ClerkMiddleware<J> {
	pub fn new(jwks_provider: J, routes: Option<Vec<String>>, validate_session_cookie: bool) -> Self {
		let authorizer = ClerkAuthorizer::new(jwks_provider, validate_session_cookie);
		Self { authorizer, routes }
	}
}

impl<S: 'static, B, J> Transform<S, ServiceRequest> for ClerkMiddleware<J>
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
	J: JwksProvider + 'static,
{
	type Response = ServiceResponse<EitherBody<B>>;
	type Error = Error;
	type InitError = ();
	type Transform = ClerkMiddlewareService<S, J>;
	type Future = Ready<Result<Self::Transform, Self::InitError>>;

	fn new_transform(&self, service: S) -> Self::Future {
		ready(Ok(ClerkMiddlewareService {
			service: Rc::new(service),
			authorizer: self.authorizer.clone(),
			routes: self.routes.clone(),
		}))
	}
}

pub struct ClerkMiddlewareService<S, J> {
	service: Rc<S>,
	authorizer: ClerkAuthorizer<J>,
	routes: Option<Vec<String>>,
}

impl<S: 'static, B, J> Service<ServiceRequest> for ClerkMiddlewareService<S, J>
where
	S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
	S::Future: 'static,
	B: 'static,
	J: JwksProvider + 'static,
{
	type Response = ServiceResponse<EitherBody<B>>;
	type Error = Error;
	type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

	forward_ready!(service);

	fn call(&self, request: ServiceRequest) -> Self::Future {
		let svc = self.service.clone();
		let authorizer = self.authorizer.clone();

		// We want to skip running the validator if we are not able to find a matching path from the listed valid paths provided by the user
		match &self.routes {
			Some(route_matches) => {
				// If the user only wants to apply authentication to a select amount of routes, we handle that logic here
				let path = request.path();
				// Check if the path was NOT contained inside of the routes specified by the user...
				let path_not_in_specified_routes = route_matches.iter().find(|&route| route == path).is_none();

				if path_not_in_specified_routes {
					// Since the path was not inside of the listed routes we want to trigger an early exit
					return Box::pin(async move {
						let res = svc.call(request).await?;
						return Ok(res.map_into_left_body());
					});
				}
			}
			// Since we did find a matching route we can simply do nothing here and start the actual auth logic...
			None => {}
		}

		Box::pin(async move {
			// Check if the request is authenticated
			match authorizer.authorize(&request).await {
				// We have authed request and can pass the user onto the next body
				Ok(jwt) => {
					request.extensions_mut().insert(jwt);
					let res = svc.call(request).await?;
					return Ok(res.map_into_left_body());
				}
				// Output any other errors thrown from the Clerk authorizer
				Err(error) => {
					match error {
						ClerkError::Unauthorized(msg) => {
							return Ok(ServiceResponse::new(
								request.into_parts().0,
								HttpResponse::Unauthorized().body(msg).map_into_right_body(),
							))
						}
						ClerkError::InternalServerError(msg) => {
							return Ok(ServiceResponse::new(
								request.into_parts().0,
								HttpResponse::InternalServerError().body(msg).map_into_right_body(),
							))
						}
					};
				}
			}
		})
	}
}
