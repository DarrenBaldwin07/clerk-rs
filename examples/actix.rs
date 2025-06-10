use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
	clerk::Clerk,
	validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
	ClerkConfiguration,
};
use dotenv::dotenv;

async fn index() -> impl Responder {
	"Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// Load environment variables from .env file if it exists
	dotenv().ok();
	
	HttpServer::new(|| {
		// Load environment variables inside the closure as well
		dotenv().ok();
		
		// Create a new clerk configuration from environment variable
		let config = ClerkConfiguration::from_env().expect("CLERK_SECRET_KEY environment variable not set");
		let clerk = Clerk::new(config);

		App::new()
			.wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
			.route("/index", web::get().to(index))
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
