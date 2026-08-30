use clerk_rs::Clerk;

pub struct LiveCredentials {
	pub session_token: String,
	pub expected_subject: String,
	secret_key: String,
}

impl LiveCredentials {
	pub fn from_env() -> Self {
		let secret_key = required_env("CLERK_SECRET_KEY");
		assert!(
			secret_key.starts_with("sk_test_"),
			"live validator tests only accept a development Clerk secret key beginning with sk_test_"
		);

		let session_token = required_env("CLERK_SESSION_TOKEN");
		assert_eq!(
			session_token.matches('.').count(),
			2,
			"CLERK_SESSION_TOKEN must be the raw JWT, without a Bearer prefix"
		);

		Self {
			secret_key,
			session_token,
			expected_subject: required_env("CLERK_TEST_USER_ID"),
		}
	}

	pub fn clerk(&self) -> Clerk {
		Clerk::from_secret_key(self.secret_key.clone())
	}

	pub fn authorization(&self) -> String {
		format!("Bearer {}", self.session_token)
	}

	pub fn assert_subject(&self, actual: &str) {
		assert_eq!(actual, self.expected_subject, "middleware extracted an unexpected JWT subject");
	}
}

fn required_env(name: &str) -> String {
	let value = std::env::var(name).unwrap_or_else(|_| panic!("{name} must be set; see tests/README.md"));
	let value = value.trim().to_owned();
	assert!(!value.is_empty(), "{name} must not be empty");
	value
}
