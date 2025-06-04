use clerk_rs::{
	clerk::Clerk,
	validators::{
		jwks::MemoryCacheJwksProvider,
		rocket::{ClerkGuard, ClerkGuardConfig},
	},
	ClerkConfiguration,
};
use dotenv::dotenv;
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
	// Load environment variables from .env file
	dotenv().ok();
	
	// Get the secret key from environment variable or provide an error message
	let secret_key = env::var("CLERK_SECRET_KEY")
		.expect("CLERK_SECRET_KEY environment variable is not set");
		
	let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
	let clerk = Clerk::new(config);
	let clerk_config = ClerkGuardConfig::new(
		MemoryCacheJwksProvider::new(clerk),
		None,
		true, // validate_session_cookie
	);

	rocket::build().mount("/", routes![index]).manage(clerk_config)
}
