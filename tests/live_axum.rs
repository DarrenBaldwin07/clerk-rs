#![cfg(feature = "axum")]

mod support;

use axum::{
	body::{to_bytes, Body},
	extract::{Extension, Request},
	http::{header, StatusCode},
	routing::get,
	Router,
};
use clerk_rs::validators::{authorizer::ClerkJwt, axum::ClerkLayer, jwks::MemoryCacheJwksProvider};
use support::LiveCredentials;
use tower::ServiceExt;

async fn protected(Extension(jwt): Extension<ClerkJwt>) -> String {
	jwt.sub
}

async fn public() -> &'static str {
	"public"
}

async fn response_text(response: axum::response::Response) -> String {
	let bytes = to_bytes(response.into_body(), 64 * 1024).await.expect("response body should be readable");
	String::from_utf8(bytes.to_vec()).expect("response body should be UTF-8")
}

#[tokio::test]
#[ignore = "requires live development Clerk credentials; see tests/README.md"]
async fn axum_validator_accepts_live_bearer_and_cookie_tokens() {
	let credentials = LiveCredentials::from_env();
	let app = Router::new()
		.route("/protected", get(protected))
		.route("/public", get(public))
		.layer(ClerkLayer::new(
			MemoryCacheJwksProvider::new(credentials.clerk()),
			Some(vec!["/protected".to_owned()]),
			true,
		));

	let response = app
		.clone()
		.oneshot(Request::builder().uri("/public").body(Body::empty()).unwrap())
		.await
		.unwrap();
	assert_eq!(response.status(), StatusCode::OK);

	let response = app
		.clone()
		.oneshot(Request::builder().uri("/protected").body(Body::empty()).unwrap())
		.await
		.unwrap();
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

	let response = app
		.clone()
		.oneshot(
			Request::builder()
				.uri("/protected")
				.header(header::AUTHORIZATION, "Bearer not-a-jwt")
				.body(Body::empty())
				.unwrap(),
		)
		.await
		.unwrap();
	assert_eq!(response.status(), StatusCode::UNAUTHORIZED);

	let response = app
		.clone()
		.oneshot(
			Request::builder()
				.uri("/protected")
				.header(header::AUTHORIZATION, credentials.authorization())
				.body(Body::empty())
				.unwrap(),
		)
		.await
		.unwrap();
	assert_eq!(response.status(), StatusCode::OK);
	credentials.assert_subject(&response_text(response).await);

	let response = app
		.oneshot(
			Request::builder()
				.uri("/protected")
				.header(header::COOKIE, format!("__session={}", credentials.session_token))
				.body(Body::empty())
				.unwrap(),
		)
		.await
		.unwrap();
	assert_eq!(response.status(), StatusCode::OK);
	credentials.assert_subject(&response_text(response).await);
}
