use clerk_rs::{
	clerk::Clerk,
	get_clerk_secret_key_or,
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
	// Get the clerk secret key from the CLERK_SECRET_KEY environment variable
	// Provide a placeholder value for development, but in production you should always use an env var
	let secret_key = Some(get_clerk_secret_key_or("placeholder_key_replace_in_production"));
	
	let config = ClerkConfiguration::new(None, None, secret_key, None);
	let clerk = Clerk::new(config);
	let clerk_config = ClerkGuardConfig::new(
		MemoryCacheJwksProvider::new(clerk),
		None,
		true, // validate_session_cookie
	);

	rocket::build().mount("/", routes![index]).manage(clerk_config)
}
