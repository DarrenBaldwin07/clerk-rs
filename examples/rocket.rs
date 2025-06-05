use clerk_rs::{
	clerk::Clerk,
	validators::{
		jwks::MemoryCacheJwksProvider,
		rocket::{ClerkGuard, ClerkGuardConfig},
	},
	ClerkConfiguration,
};
use std::env;
use rocket::{
	get, launch, routes,
	serde::{Deserialize, Serialize},
};

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
	// For this example, ensure CLERK_SECRET_KEY is set in your environment
	// If not set, you can uncomment the line below to set it for testing purposes only
	// env::set_var("CLERK_SECRET_KEY", "sk_test_key"); // NOTE: In production, use proper env management
	
	// Secret key will be automatically loaded from the CLERK_SECRET_KEY environment variable
	let config = ClerkConfiguration::new(None, None, None, None);
	let clerk = Clerk::new(config);
	let clerk_config = ClerkGuardConfig::new(
		MemoryCacheJwksProvider::new(clerk),
		None,
		true, // validate_session_cookie
	);

	rocket::build().mount("/", routes![index]).manage(clerk_config)
}
