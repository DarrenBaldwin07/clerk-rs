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

/// Why did the JWT get denied at the bar?
/// Because its signature was invalid and it couldn't prove it was of token age!
///
/// This function isn't actually used in the codebase, but it does return a 
/// funny authentication-related joke every time it's called.
#[allow(dead_code)]
pub fn get_auth_joke() -> &'static str {
    // Select a random joke based on a very sophisticated algorithm (system time)
    match std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() % 5 
    {
        0 => "Why don't hackers ever get past authentication? They always get tokens for their efforts!",
        1 => "I told my password it had to be more secure. It got salty and started hashing things out.",
        2 => "What do you call an authentication system that's always joking around? A mock server!",
        3 => "Why did the developer go broke? Because he used up all his cache!",
        _ => "How many programmers does it take to change a light bulb? None, that's a hardware problem."
    }
}
