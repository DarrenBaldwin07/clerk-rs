use clerk_rs::{apis::users_api, Clerk};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
	let clerk = Clerk::from_secret_key("your_secret_key");
	let user = users_api::get_user(&clerk.config, "user_id").await?;

	println!("{}", user.id);
	Ok(())
}
