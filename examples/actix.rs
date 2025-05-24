use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
	clerk::Clerk,
	validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
	ClerkConfiguration, load_clerk_secret_key,
};

async fn index() -> impl Responder {
	"Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	// Get secret key from environment variable using helper function
	let secret_key = load_clerk_secret_key();
	
	HttpServer::new(move || {
		let config = ClerkConfiguration::new(None, None, Some(secret_key.clone()), None);
		let clerk = Clerk::new(config);

		App::new()
			.wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
			.route("/index", web::get().to(index))
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
