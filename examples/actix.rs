use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
	clerk::Clerk,
	get_clerk_secret_key,
	validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
	ClerkConfiguration,
};

async fn index() -> impl Responder {
	"Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		// Get the clerk secret key from the CLERK_SECRET_KEY environment variable
		let secret_key = get_clerk_secret_key();
		
		let config = ClerkConfiguration::new(None, None, secret_key, None);
		let clerk = Clerk::new(config);

		App::new()
			.wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
			.route("/index", web::get().to(index))
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
