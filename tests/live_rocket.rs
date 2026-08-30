#![cfg(feature = "rocket")]

mod support;

use clerk_rs::validators::{
	jwks::MemoryCacheJwksProvider,
	rocket::{ClerkGuard, ClerkGuardConfig},
};
use rocket::{
	get,
	http::{Cookie, Header, Status},
	local::asynchronous::Client,
	routes, Rocket,
};
use support::LiveCredentials;

#[get("/protected")]
fn protected(guard: ClerkGuard<MemoryCacheJwksProvider>) -> String {
	guard.jwt.expect("ClerkGuard should contain a JWT on protected routes").sub
}

#[get("/public")]
fn public(guard: ClerkGuard<MemoryCacheJwksProvider>) -> &'static str {
	assert!(guard.jwt.is_none(), "excluded routes should not be authorized");
	"public"
}

fn test_rocket(credentials: &LiveCredentials) -> Rocket<rocket::Build> {
	let config = ClerkGuardConfig::new(
		MemoryCacheJwksProvider::new(credentials.clerk()),
		Some(vec!["/protected".to_owned()]),
		true,
	);

	rocket::build().manage(config).mount("/", routes![protected, public])
}

#[rocket::async_test]
#[ignore = "requires live development Clerk credentials; see tests/README.md"]
async fn rocket_validator_accepts_live_bearer_and_cookie_tokens() {
	let credentials = LiveCredentials::from_env();
	let client = Client::tracked(test_rocket(&credentials)).await.expect("test Rocket should launch");

	let response = client.get("/public").dispatch().await;
	assert_eq!(response.status(), Status::Ok);

	let response = client.get("/protected").dispatch().await;
	assert_eq!(response.status(), Status::Unauthorized);

	let response = client
		.get("/protected")
		.header(Header::new("Authorization", "Bearer not-a-jwt"))
		.dispatch()
		.await;
	assert_eq!(response.status(), Status::Unauthorized);

	let response = client
		.get("/protected")
		.header(Header::new("Authorization", credentials.authorization()))
		.dispatch()
		.await;
	assert_eq!(response.status(), Status::Ok);
	credentials.assert_subject(&response.into_string().await.expect("response should have a body"));

	let response = client
		.get("/protected")
		.cookie(Cookie::new("__session", credentials.session_token.clone()))
		.dispatch()
		.await;
	assert_eq!(response.status(), Status::Ok);
	credentials.assert_subject(&response.into_string().await.expect("response should have a body"));
}
