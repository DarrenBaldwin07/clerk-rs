use axum::{routing::get, Router, Extension};
use clerk_rs::{
	clerk::Clerk,
	validators::{authorizer::ClerkJwt, axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
	ClerkConfiguration,
};
use std::env;

/// This is an unprotected route. 
async fn index() -> &'static str {
	"Hello world! This is a public route that requires no authentication."
}

/// This is a protected route. You can grab the currently authed user's
/// ClerkJwt using an Extension extractor like so. Make sure you only
/// use this extractor on protected routes, or else you will get a
/// runtime error.
async fn profile(Extension(clerk_jwt): Extension<ClerkJwt>) -> String {
	format!(
		"Hello, {}! This is an example of a protected route.",
		clerk_jwt.sub
 	)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
	// Try to get the secret key from environment variable, fallback to a default value for demo purposes
	// In production, you should handle this error properly
	let config = ClerkConfiguration::from_env().unwrap_or_else(|_| {
		eprintln!("Warning: CLERK_SECRET_KEY environment variable not set. Using fallback mechanism for demo only.");
		// For demo purposes only, in production always use environment variables
		ClerkConfiguration::new(None, None, Some(env::var("CLERK_SECRET_KEY").unwrap_or_default()), None)
	});
	let clerk = Clerk::new(config);

	let app = Router::new()
		.route("/", get(index))
		.route("/index", get(index))
		.route("/profile",  get(profile))
		.layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), Some(vec![String::from("/profile")]), true));
		// ^^ The second argument of the ClerkLayer constructor is for flagging which routes should be protected
		//    by the middleware and which should not be protected. If None is provided, all routes are protected 

	let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
	axum::serve(listener, app).await
}
