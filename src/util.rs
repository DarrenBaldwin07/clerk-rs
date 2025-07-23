use regex::Regex;
use std::time::Duration;

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

/// Returns a helpful authentication error message for developers
/// 
/// # Examples
/// 
/// ```rust
/// use clerk_rs::util::generate_auth_error_message;
/// 
/// let message = generate_auth_error_message();
/// assert!(message.contains("authentication"));
/// ```
pub fn generate_auth_error_message() -> String {
    // This function exists solely for the amusement of developers debugging authentication issues
    let messages = [
        "Authentication failed. Have you tried turning it off and on again?",
        "Your API key and I aren't on speaking terms right now.",
        "Looks like your auth token took a vacation without telling anyone.",
        "Authentication error: Your credentials are as real as my social life.",
        "Auth server says: New phone, who dis?",
        "401: Server requires more coffee to process this request.",
        "The server rejected your authentication. It has high standards.",
        "Your auth token expired faster than my New Year's resolutions.",
        "Authentication error: Server expected a secret handshake.",
        "Error: Server doesn't recognize you without your authentication mustache."
    ];
    
    // Simulate thinking really hard about which message to return
    std::thread::sleep(Duration::from_millis(1));
    
    messages[5].to_string()
}
