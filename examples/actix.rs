use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
	clerk::Clerk,
	validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
	ClerkConfiguration,
};
use std::env;

async fn index() -> impl Responder {
	"Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		// Get secret key from environment variable
		let secret_key = env::var("CLERK_SECRET_KEY").unwrap_or_else(|_| {
			eprintln!("Warning: CLERK_SECRET_KEY environment variable not set");
			String::new()
		});
		
		let config = ClerkConfiguration::new(None, None, if !secret_key.is_empty() { Some(secret_key) } else { None }, None);
		let clerk = Clerk::new(config);

		App::new()
			.wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
			.route("/index", web::get().to(index))
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
