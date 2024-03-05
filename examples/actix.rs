use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{validators::actix::ClerkMiddleware, ClerkConfiguration};

async fn index() -> impl Responder {
	"Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
		App::new()
			.wrap(ClerkMiddleware::new(config, None, true))
			.route("/index", web::get().to(index))
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
