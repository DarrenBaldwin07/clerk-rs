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
	// For this example, ensure CLERK_SECRET_KEY is set in your environment
	// If not set, you can uncomment the line below to set it for testing purposes only
	// env::set_var("CLERK_SECRET_KEY", "your_secret_key"); // NOTE: In production, use proper env management
	
	HttpServer::new(|| {
		// Secret key will be automatically loaded from the CLERK_SECRET_KEY environment variable
		let config = ClerkConfiguration::new(None, None, None, None);
		let clerk = Clerk::new(config);

		App::new()
			.wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
			.route("/index", web::get().to(index))
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
