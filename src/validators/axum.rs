use super::authorizer::{ClerkAuthorizer, ClerkError, ClerkRequest};
use crate::{clerk::Clerk, ClerkConfiguration};
use axum::{
	body::Body,
	extract::Request,
	http::{HeaderMap, HeaderValue, StatusCode},
	response::Response,
};
use axum_extra::extract::cookie::CookieJar;
use futures_util::future::BoxFuture;
use std::task::{Context, Poll};
use tower::{Layer, Service};

pub struct AxumClerkRequest {
	pub headers: HeaderMap<HeaderValue>,
}

impl AxumClerkRequest {
	pub fn from_axum_request(request: &Request) -> Self {
		Self {
			headers: request.headers().clone(),
		}
	}
}

impl ClerkRequest for AxumClerkRequest {
	fn get_header(&self, key: &str) -> Option<String> {
		match self.headers.get(key) {
			Some(val) => match val.to_str() {
				Ok(val) => Some(val.to_string()),
				Err(_) => None,
			},
			None => None,
		}
	}

	fn get_cookie(&self, key: &str) -> Option<String> {
		match CookieJar::from_headers(&self.headers).get(key) {
			Some(val) => Some(val.value().to_string()),
			None => None,
		}
	}
}

/// Axum layer for protecting a http endpoint with Clerk.dev
/// # Example
/// ```
/// async fn index() -> &'static str {
/// 	"Hello world!"
/// }
///
/// #[tokio::main]
/// async fn main() -> std::io::Result<()> {
/// 	let config: ClerkConfiguration = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
/// 	let app = Router::new().route("/index", get(index)).layer(ClerkLayer::new(config, None, true));
/// 	let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
/// 	let listener = tokio::net::TcpListener::bind(addr).await?;
/// 	axum::serve(listener, app).await
/// }
/// ```
#[derive(Clone)]
pub struct ClerkLayer {
	pub clerk_config: ClerkConfiguration,
	routes: Option<Vec<String>>,
	pub should_validate_session_cookie: bool,
}

impl ClerkLayer {
	pub fn new(clerk_config: ClerkConfiguration, routes: Option<Vec<String>>, should_validate_session_cookie: bool) -> Self {
		ClerkLayer {
			clerk_config,
			routes,
			should_validate_session_cookie,
		}
	}
}

impl<S> Layer<S> for ClerkLayer {
	type Service = ClerkMiddleware<S>;

	fn layer(&self, service: S) -> Self::Service {
		ClerkMiddleware {
			service,
			authorizer: ClerkAuthorizer::new(Clerk::new(self.clerk_config.clone()), self.should_validate_session_cookie),
			routes: self.routes.clone(),
		}
	}
}

#[derive(Clone)]
pub struct ClerkMiddleware<S> {
	service: S,
	authorizer: ClerkAuthorizer,
	routes: Option<Vec<String>>,
}

impl<S> Service<Request> for ClerkMiddleware<S>
where
	S: Service<Request, Response = Response> + Send + 'static + Clone,
	S::Future: Send + 'static,
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

	fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		self.service.poll_ready(cx)
	}

	fn call(&mut self, request: Request) -> Self::Future {
		let mut svc = self.service.clone();

		// We want to skip running the validator if we are not able to find a matching path from the listed valid paths provided by the user
		match &self.routes {
			Some(route_matches) => {
				// If the user only wants to apply authentication to a select amount of routes, we handle that logic here
				let path = request.uri().path();
				// Check if the path was NOT contained inside of the routes specified by the user...
				let path_not_in_specified_routes = route_matches.iter().find(|&route| route == path).is_none();

				if path_not_in_specified_routes {
					// Since the path was not inside of the listed routes we want to trigger an early exit
					return Box::pin(async move {
						let res = svc.call(request).await?;
						return Ok(res);
					});
				}
			}
			// Since we did find a matching route we can simply do nothing here and start the actual auth logic...
			None => {}
		}

		let authorizer = self.authorizer.clone();
		let req = AxumClerkRequest::from_axum_request(&request);

		Box::pin(async move {
			// Check if the request is authenticated
			match authorizer.authorize(&req).await {
				// We have authed request and can pass the user onto the next body
				Ok(_) => {
					let res = svc.call(request).await?;
					return Ok(res);
				}
				// Output any other errors thrown from the Clerk authorizer
				Err(error) => {
					match error {
						ClerkError::Unauthorized(msg) => {
							return Ok(Response::builder().status(StatusCode::UNAUTHORIZED).body(Body::from(msg)).unwrap())
						}
						ClerkError::InternalServerError(msg) => {
							return Ok(Response::builder()
								.status(StatusCode::INTERNAL_SERVER_ERROR)
								.body(Body::from(msg))
								.unwrap())
						}
					};
				}
			}
		})
	}
}
