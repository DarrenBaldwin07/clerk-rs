use crate::validators::{
	authorizer::{ClerkAuthorizer, ClerkError, ClerkRequest},
	jwks::JwksProvider,
};
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

/// Axum layer for protecting a http endpoint with Clerk.dev.
///
/// Supports dynamic path matching using curly braces syntax (e.g., `/api/users/{user_id}`).
/// When specifying routes, you can use dynamic segments that will match any non-empty path segment.
///
/// # Example
/// ```
/// async fn index() -> &'static str {
///     "Hello world!"
/// }
///
/// #[tokio::main]
/// async fn main() -> std::io::Result<()> {
///     let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
///     let clerk = Clerk::new(config);
///
///     // Protect specific routes, including dynamic paths
///     let protected_routes = vec![
///         "/api/users".to_string(),
///         "/api/users/{user_id}".to_string(),
///         "/api/users/{user_id}/profile".to_string(),
///     ];
///
///     let app = Router::new()
///         .route("/index", get(index))
///         .layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), Some(protected_routes), true));
///
///     let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
///     axum::serve(listener, app).await
/// }
/// ```
pub struct ClerkLayer<J> {
	authorizer: ClerkAuthorizer<J>,
	routes: Option<Vec<String>>,
}

impl<J: JwksProvider> ClerkLayer<J> {
	pub fn new(jwks_provider: J, routes: Option<Vec<String>>, validate_session_cookie: bool) -> Self {
		let authorizer = ClerkAuthorizer::new(jwks_provider, validate_session_cookie);
		Self { authorizer, routes }
	}
}

impl<S, J> Layer<S> for ClerkLayer<J> {
	type Service = ClerkMiddleware<S, J>;

	fn layer(&self, service: S) -> Self::Service {
		ClerkMiddleware {
			service,
			authorizer: self.authorizer.clone(),
			routes: self.routes.clone(),
		}
	}
}

impl<J> Clone for ClerkLayer<J> {
	fn clone(&self) -> Self {
		Self {
			authorizer: self.authorizer.clone(),
			routes: self.routes.clone(),
		}
	}
}

pub struct ClerkMiddleware<S, J> {
	service: S,
	authorizer: ClerkAuthorizer<J>,
	routes: Option<Vec<String>>,
}

impl<S, J> Service<Request> for ClerkMiddleware<S, J>
where
	S: Service<Request, Response = Response> + Send + 'static + Clone,
	S::Future: Send + 'static,
	J: JwksProvider + Send + Sync + 'static,
{
	type Response = S::Response;
	type Error = S::Error;
	type Future = BoxFuture<'static, Result<Self::Response, Self::Error>>;

	fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
		self.service.poll_ready(cx)
	}

	fn call(&mut self, mut request: Request) -> Self::Future {
		let mut svc = self.service.clone();

		// We want to skip running the validator if we are not able to find a matching path from the listed valid paths provided by the user
		match &self.routes {
			Some(route_matches) => {
				// If the user only wants to apply authentication to a select amount of routes, we handle that logic here
				let path = request.uri().path();
				// Check if the path was NOT contained inside of the routes specified by the user...
				let path_not_in_specified_routes = route_matches.iter().find(|&route| path_matches_pattern(route, path)).is_none();

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
				Ok(jwt) => {
					request.extensions_mut().insert(jwt);
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

	/// route_pattern example: "/api/users/{user_id}/profile"
	/// actual_path example: "/api/users/fc428b58-50ad-45d2-8729-e50ecded3a7b/profile"
	fn path_matches_pattern(route_pattern: &str, actual_path: &str) -> bool {
		let pattern_segments: Vec<&str> = route_pattern.split('/').collect();
		let path_segments: Vec<&str> = actual_path.split('/').collect();

		if pattern_segments.len() != path_segments.len() {
			return false;
		}

		for (pattern_seg, path_seg) in pattern_segments.iter().zip(path_segments.iter()) {
			if pattern_seg.starts_with('{') && pattern_seg.ends_with('}') {
				if path_seg.is_empty() {
					return false;
				}
				continue;
			} else {
				if pattern_seg != path_seg {
					return false;
				}
			}
		}

		true
	}
}

impl<S: Clone, J> Clone for ClerkMiddleware<S, J> {
	fn clone(&self) -> Self {
		Self {
			service: self.service.clone(),
			authorizer: self.authorizer.clone(),
			routes: self.routes.clone(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_path_matches_pattern() {
		// Dynamic segment matches
		assert!(path_matches_pattern(
			"/api/users/{user_id}/profile",
			"/api/users/fc428b58-50ad-45d2-8729-e50ecded3a7b/profile"
		));

		// Static segment matches
		assert!(path_matches_pattern("/api/health", "/api/health"));
		assert!(!path_matches_pattern("/api/health", "/api/status"));

		// Multiple dynamic segments match
		assert!(path_matches_pattern(
			"/api/users/{user_id}/profile/{profile_id}",
			"/api/users/123/profile/456"
		));

		// Static segment does not match
		assert!(!path_matches_pattern(
			"/api/users/{user_id}/profile",
			"/api/users/fc428b58-50ad-45d2-8729-e50ecded3a7b/detail"
		));

		// The number of segments does not match
		assert!(!path_matches_pattern(
			"/api/users/{user_id}/profile",
			"/api/users/fc428b58-50ad-45d2-8729-e50ecded3a7b"
		));

		// Empty dynamic segment
		assert!(!path_matches_pattern("/api/users/{user_id}/profile", "/api/users//profile"));
	}
}
