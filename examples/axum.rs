use axum::{routing::get, Router};
use clerk_rs::{
	clerk::Clerk,
	validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
	ClerkConfiguration,
};

async fn index() -> &'static str {
	"Hello world!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
	let clerk = Clerk::new(config);

	let app = Router::new()
		.route("/index", get(index))
		.layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), None, true));

	let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
	axum::serve(listener, app).await
}
