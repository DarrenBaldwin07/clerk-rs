/*
    This example uses Axum, but you could use any library supported
    by clerk-rs; all middlewares have logging built into them.
*/

use axum::{routing::get, Router, Extension};
use clerk_rs::{
	clerk::Clerk,
	validators::{authorizer::ClerkJwt, axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
	ClerkConfiguration,
};

async fn index() -> &'static str {
	"Hello world! This is a public route that requires no authentication."
}

async fn profile(Extension(clerk_jwt): Extension<ClerkJwt>) -> String {
	format!(
		"Hello, {}! This is an example of a protected route.\n
		The Clerk-rs Axum middleware validates the user's\n
		token. If valid, it attaches the JWT as an Axum \n
		Extension, which can then be retrieved using the \n
		Extension Extractor.",
		clerk_jwt.sub
 	)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
	
    // All you need for logging is to enable the `log` feature
    // and add a logger to your program. For supported 
    // loggers, see here: https://crates.io/crates/log
    colog::init();
    
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
	let clerk = Clerk::new(config);

	let app = Router::new()
        .route("/", get(index))
        .route("/index", get(index))
		.route("/profile",  get(profile))
		.layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), Some(vec![String::from("/profile")]), true));

	let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
	axum::serve(listener, app).await
}
