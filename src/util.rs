use regex::Regex;
use std::env;

/// Method for converting a dynamic query string (ex: /getUser/{id}) and converting it to something like: "/getUser/1741897489174891"
pub fn generate_path_from_params(route_path: String, params: Vec<&str>) -> String {
	let dynamic_regex = Regex::new(r"\{[^\{\}]*\}").unwrap();
	let mut matches: Vec<String> = Vec::new();
	let mut new_route_path = route_path;

	// Get every matched segment
	for capture in dynamic_regex.captures_iter(&new_route_path) {
		let dynamic_segement = capture[0].to_string();
		matches.push(dynamic_segement);
	}

	// Now that we have every match lets replace every string in the route path with user specified route params
	for (index, str_match) in matches.iter().enumerate() {
		let parsed_path = new_route_path.replacen(str_match, &params[index], 1);
		new_route_path = parsed_path;
	}

	new_route_path
}

/// Gets the Clerk secret key from environment variables
/// The key is expected to be stored in the CLERK_SECRET_KEY environment variable
/// 
/// # Examples
/// 
/// ```rust
/// use clerk_rs::util::get_clerk_secret_key;
/// 
/// // This will look for the CLERK_SECRET_KEY environment variable
/// let secret_key = get_clerk_secret_key();
/// 
/// // You can also specify a fallback value
/// let secret_key = get_clerk_secret_key_or("fallback_key");
/// ```
pub fn get_clerk_secret_key() -> Option<String> {
    env::var("CLERK_SECRET_KEY").ok()
}

/// Gets the Clerk secret key from environment variables with a fallback value
/// The key is expected to be stored in the CLERK_SECRET_KEY environment variable
/// If the environment variable is not set, the fallback value is returned
pub fn get_clerk_secret_key_or(fallback: &str) -> String {
    env::var("CLERK_SECRET_KEY").unwrap_or_else(|_| fallback.to_string())
}
