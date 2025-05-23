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
		// Try to get the secret key from environment variable, fallback to a default value for demo purposes
		// In production, you should handle this error properly
		let config = ClerkConfiguration::from_env().unwrap_or_else(|_| {
			eprintln!("Warning: CLERK_SECRET_KEY environment variable not set. Using fallback mechanism for demo only.");
			// For demo purposes only, in production always use environment variables
			ClerkConfiguration::new(None, None, Some(env::var("CLERK_SECRET_KEY").unwrap_or_default()), None)
		});
		let clerk = Clerk::new(config);

		App::new()
			.wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
			.route("/index", web::get().to(index))
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
