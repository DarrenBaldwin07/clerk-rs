#![cfg(feature = "actix")]

mod support;

use actix_web::{
	http::{header, StatusCode},
	test, web, App, HttpMessage, HttpRequest, HttpResponse,
};
use clerk_rs::validators::{actix::ClerkMiddleware, authorizer::ClerkJwt, jwks::MemoryCacheJwksProvider};
use support::LiveCredentials;

async fn protected(request: HttpRequest) -> HttpResponse {
	let subject = request
		.extensions()
		.get::<ClerkJwt>()
		.expect("ClerkMiddleware should attach ClerkJwt to protected requests")
		.sub
		.clone();
	HttpResponse::Ok().body(subject)
}

async fn public() -> HttpResponse {
	HttpResponse::Ok().body("public")
}

#[actix_web::test]
#[ignore = "requires live development Clerk credentials; see tests/README.md"]
async fn actix_validator_accepts_live_bearer_and_cookie_tokens() {
	let credentials = LiveCredentials::from_env();
	let app = test::init_service(
		App::new()
			.wrap(ClerkMiddleware::new(
				MemoryCacheJwksProvider::new(credentials.clerk()),
				Some(vec!["/protected".to_owned()]),
				true,
			))
			.route("/protected", web::get().to(protected))
			.route("/public", web::get().to(public)),
	)
	.await;

	let response = test::call_service(&app, test::TestRequest::get().uri("/public").to_request()).await;
	assert_eq!(response.status(), StatusCode::OK);

	let response = test::call_service(&app, test::TestRequest::get().uri("/protected").to_request()).await;
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

	let response = test::call_service(
		&app,
		test::TestRequest::get()
			.uri("/protected")
			.insert_header((header::AUTHORIZATION, "Bearer not-a-jwt"))
			.to_request(),
	)
	.await;
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

	let response = test::call_service(
		&app,
		test::TestRequest::get()
			.uri("/protected")
			.insert_header((header::AUTHORIZATION, credentials.authorization()))
			.to_request(),
	)
	.await;
	assert_eq!(response.status(), StatusCode::OK);
	let body = test::read_body(response).await;
	credentials.assert_subject(std::str::from_utf8(&body).expect("subject response should be UTF-8"));

	let response = test::call_service(
		&app,
		test::TestRequest::get()
			.uri("/protected")
			.cookie(actix_web::cookie::Cookie::new("__session", credentials.session_token.clone()))
			.to_request(),
	)
	.await;
	assert_eq!(response.status(), StatusCode::OK);
	let body = test::read_body(response).await;
	credentials.assert_subject(std::str::from_utf8(&body).expect("subject response should be UTF-8"));
}
