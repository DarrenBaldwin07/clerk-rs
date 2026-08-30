use clerk_rs::{apis::users_api, ClerkConfiguration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let config = ClerkConfiguration::new("your_secret_key");
	let user = users_api::get_user(&config, "user_id").await?;

	println!("{}", user.id);
	Ok(())
}
