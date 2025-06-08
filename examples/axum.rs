use std::sync::LazyLock;

use axum::{routing::get, Router};
use clerk_rs::{
	clerk::Clerk,
	validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
	ClerkConfiguration,
};

static CLERK_INSTANCE: LazyLock<Clerk> = LazyLock::new(|| {
	let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
	Clerk::new(config)
});

async fn index() -> &'static str {
	"Hello world!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let app = Router::new()
		.route("/index", get(index))
		.layer(ClerkLayer::new(MemoryCacheJwksProvider::new(&CLERK_INSTANCE), true));

	let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
	axum::serve(listener, app).await
}
