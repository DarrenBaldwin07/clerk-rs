#![cfg(feature = "poem")]

mod support;

use clerk_rs::validators::{authorizer::ClerkJwt, jwks::MemoryCacheJwksProvider, poem::ClerkPoemMiddleware};
use poem::{
	get, handler,
	http::{header, StatusCode},
	web::Data,
	Endpoint, EndpointExt, Request, Route,
};
use support::LiveCredentials;

#[handler]
fn protected(Data(jwt): Data<&ClerkJwt>) -> String {
	jwt.sub.clone()
}

#[handler]
fn public() -> &'static str {
	"public"
}

#[tokio::test]
#[ignore = "requires live development Clerk credentials; see tests/README.md"]
async fn poem_validator_accepts_live_bearer_and_cookie_tokens() {
	let credentials = LiveCredentials::from_env();
	let app = Route::new()
		.at("/protected", get(protected))
		.at("/public", get(public))
		.with(ClerkPoemMiddleware::new(
			MemoryCacheJwksProvider::new(credentials.clerk()),
			true,
			Some(vec!["/public".to_owned()]),
		));

	let response = app.get_response(Request::builder().uri("/public".parse().unwrap()).finish()).await;
	assert_eq!(response.status(), StatusCode::OK);

	let response = app.get_response(Request::builder().uri("/protected".parse().unwrap()).finish()).await;
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

	let response = app
		.get_response(
			Request::builder()
				.uri("/protected".parse().unwrap())
				.header(header::AUTHORIZATION, "Bearer not-a-jwt")
				.finish(),
		)
		.await;
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

	let response = app
		.get_response(
			Request::builder()
				.uri("/protected".parse().unwrap())
				.header(header::AUTHORIZATION, credentials.authorization())
				.finish(),
		)
		.await;
	assert_eq!(response.status(), StatusCode::OK);
	let subject = response.into_body().into_string().await.expect("response body should be readable");
	credentials.assert_subject(&subject);

	let response = app
		.get_response(
			Request::builder()
				.uri("/protected".parse().unwrap())
				.header(header::COOKIE, format!("__session={}", credentials.session_token))
				.finish(),
		)
		.await;
	assert_eq!(response.status(), StatusCode::OK);
	let subject = response.into_body().into_string().await.expect("response body should be readable");
	credentials.assert_subject(&subject);
}
