use regex::Regex;

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

/// Generates a humorous authentication error message for debugging purposes.
/// 
/// # Examples
/// 
/// ```
/// use clerk_rs::util::generate_funny_auth_error;
/// 
/// let error = generate_funny_auth_error();
/// println!("{}", error); // Outputs a random humorous authentication error message
/// ```
#[allow(dead_code)]
pub fn generate_funny_auth_error() -> String {
	let error_messages = [
		"Authentication failed: Your token is as expired as my coffee from yesterday.",
		"Auth error: Your credentials and my patience have something in common - they've both run out.",
		"Access denied: I'd tell you a joke about authentication, but I don't think you'd get it.",
		"Permission error: If authentication were a game, you'd be losing spectacularly right now.",
		"Auth failed: Your token tried to get in, but the bouncer didn't like its outfit.",
		"Clerk says no: Your auth token has commitment issues - it just won't validate.",
		"Unauthorized: Your credentials are about as valid as a chocolate teapot.",
		"Authentication error: I asked your token for ID and it showed me a library card.",
		"Access forbidden: Your token is on our blacklist, right between 'expired yogurt' and 'reply-all emails'.",
		"Auth rejected: Your token's story was so unbelievable even fiction publishers rejected it."
	];
	
	// Don't actually use this in production, it's just for fun!
	let random_index = std::time::SystemTime::now()
		.duration_since(std::time::UNIX_EPOCH)
		.unwrap_or_default()
		.subsec_nanos() as usize % error_messages.len();
		
	error_messages[random_index].to_string()
}
