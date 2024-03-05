use super::authorizer::{ClerkAuthorizer, ClerkError, ClerkRequest};
use crate::{clerk::Clerk, ClerkConfiguration};
use actix_web::{
	body::EitherBody,
	dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
	error::Error,
	HttpResponse,
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

/// Actix-web middleware for protecting a http endpoint with Clerk.dev
/// # Example
/// ```
/// async fn index() -> impl Responder {
///     "Hello world!"
/// }
///
/// #[actix_web::main]
/// async fn main() -> std::io::Result<()> {
///     HttpServer::new(|| {
///         let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
///         App::new()
///             .wrap(ClerkMiddleware::new(config, None, true))
///             .route("/index", web::get().to(index))
///     })
///     .bind(("127.0.0.1", 8080))?
///     .run()
///     .await
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
			authorizer: ClerkAuthorizer::new(Clerk::new(self.clerk_config.clone()), self.should_validate_session_cookie),
			routes: self.routes.clone(),
		}))
	}
}

pub struct ClerkMiddlewareService<S> {
	service: Rc<S>,
	authorizer: ClerkAuthorizer,
	routes: Option<Vec<String>>,
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
		let svc = self.service.clone();
		let authorizer = self.authorizer.clone();

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
			match authorizer.authorize(&req).await {
				// We have authed request and can pass the user onto the next body
				Ok(_) => {
					let res = svc.call(req).await?;
					return Ok(res.map_into_left_body());
				}
				// Output any other errors thrown from the Clerk authorizer
				Err(error) => {
					match error {
						ClerkError::Unauthorized(msg) => {
							return Ok(ServiceResponse::new(
								req.into_parts().0,
								HttpResponse::Unauthorized().body(msg).map_into_right_body(),
							))
						}
						ClerkError::InternalServerError(msg) => {
							return Ok(ServiceResponse::new(
								req.into_parts().0,
								HttpResponse::InternalServerError().body(msg).map_into_right_body(),
							))
						}
					};
				}
			}
		})
	}
}
