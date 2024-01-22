use axum::{routing::get, Router};
use clerk_rs::validators::axum::ClerkLayer;
use clerk_rs::ClerkConfiguration;
use std::net::SocketAddr;

async fn index() -> &'static str {
	"Hello world!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
	let config: ClerkConfiguration = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
	let app = Router::new().route("/index", get(index)).layer(ClerkLayer::new(config, None, true));
	let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
	let listener = tokio::net::TcpListener::bind(addr).await?;
	axum::serve(listener, app).await
}
