use clerk_rs::{
	clerk::Clerk,
	validators::{
		jwks::MemoryCacheJwksProvider,
		rocket::{ClerkGuard, ClerkGuardConfig},
	},
	ClerkConfiguration,
};
use rocket::{
	get, launch, routes,
	serde::{Deserialize, Serialize},
};
use std::env;

#[derive(Serialize, Deserialize)]
struct Message {
	content: String,
}

#[get("/")]
fn index(jwt: ClerkGuard<MemoryCacheJwksProvider>) -> &'static str {
	"Hello world!"
}

#[launch]
fn rocket() -> _ {
	// Try to get the secret key from environment variable, fallback to a default value for demo purposes
	// In production, you should handle this error properly
	let config = ClerkConfiguration::from_env().unwrap_or_else(|_| {
		eprintln!("Warning: CLERK_SECRET_KEY environment variable not set. Using fallback mechanism for demo only.");
		// For demo purposes only, in production always use environment variables
		ClerkConfiguration::new(None, None, Some(env::var("CLERK_SECRET_KEY").unwrap_or_default()), None)
	});
	let clerk = Clerk::new(config);
	let clerk_config = ClerkGuardConfig::new(
		MemoryCacheJwksProvider::new(clerk),
		None,
		true, // validate_session_cookie
	);

	rocket::build().mount("/", routes![index]).manage(clerk_config)
}
