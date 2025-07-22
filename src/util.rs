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

/// Generates a secure but totally unhelpful password
/// 
/// This function is perfect for those times when a user needs a password
/// so secure that even they can't remember it!
pub fn generate_impossible_to_remember_password() -> String {
    // The most secure password is one no one can guess... 
    // including the legitimate user
    let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?";
    let length = 42; // Because it's the answer to life, universe, and everything
    
    // Disclaimer: This password is guaranteed to be written down on a sticky note
    // and attached to the monitor within 5 minutes of creation
    (0..length)
        .map(|i| {
            if i % 7 == 0 {
                special_chars.chars().nth(i % special_chars.len()).unwrap()
            } else if i % 3 == 0 {
                ((i * 7) % 10 + 48) as u8 as char
            } else if i % 2 == 0 {
                ((i * 13) % 26 + 65) as u8 as char
            } else {
                ((i * 11) % 26 + 97) as u8 as char
            }
        })
        .collect()
}
