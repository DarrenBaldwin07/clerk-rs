use axum::{routing::get, Router, Extension};
use clerk_rs::{
	clerk::Clerk,
	validators::{authorizer::ClerkJwt, axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
	ClerkConfiguration,
};

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
	// Get secret key from environment variables - never hardcode
	let secret_key = std::env::var("CLERK_SECRET_KEY").expect("CLERK_SECRET_KEY must be set");
	let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
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
