use clerk_rs::models::{ExternalAccountWithVerificationVerification, Session};
use serde_json::json;

const MILLIS_AFTER_I32_MAX: i64 = 1_788_133_110_277;

#[test]
fn session_accepts_millisecond_last_active_timestamp() {
	let session: Session = serde_json::from_value(json!({
		"object": "session",
		"id": "sess_test",
		"user_id": "user_test",
		"client_id": "client_test",
		"actor": null,
		"status": "active",
		"last_active_organization_id": null,
		"last_active_at": MILLIS_AFTER_I32_MAX,
		"latest_activity": null,
		"expire_at": MILLIS_AFTER_I32_MAX,
		"abandon_at": MILLIS_AFTER_I32_MAX,
		"updated_at": MILLIS_AFTER_I32_MAX,
		"created_at": MILLIS_AFTER_I32_MAX,
		"tasks": null
	}))
	.expect("session millisecond timestamps should deserialize as i64");

	assert_eq!(session.last_active_at, MILLIS_AFTER_I32_MAX);
}

#[test]
fn oauth_verification_accepts_millisecond_expiration_timestamp() {
	let verification: ExternalAccountWithVerificationVerification = serde_json::from_value(json!({
		"object": "verification_oauth",
		"status": "verified",
		"strategy": "oauth_google",
		"external_verification_redirect_url": null,
		"error": null,
		"expire_at": MILLIS_AFTER_I32_MAX,
		"attempts": 1,
		"verified_at_client": null
	}))
	.expect("verification expiration timestamps should deserialize as i64");

	match verification {
		ExternalAccountWithVerificationVerification::VerificationOauth(verification) => {
			assert_eq!(verification.expire_at, MILLIS_AFTER_I32_MAX);
		}
		other => panic!("expected OAuth verification, got {other:?}"),
	}
}
