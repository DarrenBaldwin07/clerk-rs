use clerk_rs::{
	apis::{o_auth_access_tokens_api::verify_o_auth_access_token, Error},
	models::{VerifyOAuthAccessToken200Response, VerifyOAuthAccessTokenRequest},
	ClerkConfiguration,
};
use serde_json::json;

fn configuration(server: &mockito::Server) -> ClerkConfiguration {
	ClerkConfiguration::new("sk_test_example").with_base_path(format!("{}/v1", server.url()))
}

fn request(access_token: &str) -> VerifyOAuthAccessTokenRequest {
	VerifyOAuthAccessTokenRequest {
		access_token: Some(access_token.to_owned()),
		..Default::default()
	}
}

#[tokio::test]
async fn verification_returns_the_access_token() {
	let mut server = mockito::Server::new_async().await;
	let mock = server
		.mock("POST", "/v1/oauth_applications/access_tokens/verify")
		.match_header("authorization", "Bearer sk_test_example")
		.match_body(mockito::Matcher::Json(json!({ "access_token": "oat_secret" })))
		.expect(1)
		.with_header("content-type", "application/json")
		.with_body(
			json!({
				"object": "clerk_idp_oauth_access_token",
				"id": "oat_0ef5a7a33d87ed87ee7954c845d80450",
				"client_id": "client_2xhFjEI5X2qWRvtV13BzSj8H6Dk",
				"subject": "user_2xhFjEI5X2qWRvtV13BzSj8H6Dk",
				"scopes": ["read", "write"],
				"revoked": false,
				"revocation_reason": null,
				"expired": false,
				"expiration": 1716883200,
				"created_at": 1716883200,
				"updated_at": 1716883200
			})
			.to_string(),
		)
		.create_async()
		.await;

	let response = verify_o_auth_access_token(&configuration(&server), request("oat_secret"))
		.await
		.expect("an access token without an `active` property should deserialize");

	match response {
		VerifyOAuthAccessToken200Response::OAuthAccessTokenDetails(token) => {
			assert_eq!(token.id, "oat_0ef5a7a33d87ed87ee7954c845d80450");
			assert_eq!(token.subject, "user_2xhFjEI5X2qWRvtV13BzSj8H6Dk");
			assert_eq!(token.scopes, ["read", "write"]);
			assert_eq!(token.revocation_reason, None);
			assert_eq!(token.expiration, Some(1716883200.0));
		}
		other => panic!("expected an access token, got {other:?}"),
	}

	mock.assert_async().await;
}

#[tokio::test]
async fn verification_returns_an_expired_jwt() {
	let mut server = mockito::Server::new_async().await;
	server
		.mock("POST", "/v1/oauth_applications/access_tokens/verify")
		.with_header("content-type", "application/json")
		.with_body(json!({ "active": false }).to_string())
		.create_async()
		.await;

	let response = verify_o_auth_access_token(&configuration(&server), request("expired.jwt.token"))
		.await
		.expect("a response with only an `active` property should deserialize");

	match response {
		VerifyOAuthAccessToken200Response::ExpiredJwtAccessToken(expired) => assert!(!expired.active),
		other => panic!("expected an expired JWT, got {other:?}"),
	}
}

#[tokio::test]
async fn verification_rejects_an_unknown_access_token() {
	let mut server = mockito::Server::new_async().await;
	server
		.mock("POST", "/v1/oauth_applications/access_tokens/verify")
		.with_status(404)
		.with_header("content-type", "application/json")
		.with_body(
			json!({
				"errors": [{
					"message": "some details about the error",
					"long_message": "some details about the error",
					"code": "some_error_code"
				}]
			})
			.to_string(),
		)
		.create_async()
		.await;

	let error = verify_o_auth_access_token(&configuration(&server), request("oat_unknown"))
		.await
		.expect_err("an unknown access token should be rejected");

	match error {
		Error::ResponseError(response) => {
			assert_eq!(response.status.as_u16(), 404);
			assert!(response.entity.is_some(), "the documented error body should deserialize");
		}
		other => panic!("expected a response error, got {other:?}"),
	}
}
