use axum::{routing::get, Extension, Router};
use clerk_rs::{
	clerk::Clerk,
	validators::{authorizer::ClerkJwt, axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
	ClerkConfiguration,
};

async fn index() -> &'static str {
	"Hello world! This is an unprotected route!"
}

/// This route can retrieve the user's authentication information using the Extension<ClerkJwt> extractor.
/// Note: If you use this extractor on an unauthenticated route, you will hit a runtime error.
async fn protected_route(Extension(clerk_jwt): Extension<ClerkJwt>) -> String {
    format!("Hello, user with id {}! This is a protected route, you had to be authorized to get this!", clerk_jwt.sub)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {

    // Use a basic tracing_subscriber to print logs to the console.
    tracing_subscriber::fmt::init();

    // Paste your secret key over 'your_secret_key'.
    // Do note: if building a real project, you should never put secrets inside code source.
    // Consider storing secrets in environmental variables instead.
	let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
	let clerk = Clerk::new(config);

	let app = Router::new()
        .route("/", get(index))
        .route("/index", get(index))
        .route("/user", get(protected_route))
        .layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), Some(vec![String::from("/user")]), true));

	let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
	axum::serve(listener, app).await
}
