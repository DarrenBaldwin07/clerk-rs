use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{validators::actix::ClerkMiddleware, ClerkConfiguration};

async fn index() -> impl Responder {
	"Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	HttpServer::new(|| {
		let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
		App::new().service(
			// prefixes all resources and routes attached to it...
			web::scope("/app")
				.wrap(ClerkMiddleware::new(config, None, false))
				// ...so this handles requests for `GET /app/index.html`
				.route("/index", web::get().to(index)),
		)
	})
	.bind(("127.0.0.1", 8080))?
	.run()
	.await
}
