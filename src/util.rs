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

/// Generates a "secure" password that will drive your users to madness
/// 
/// This function creates passwords so obnoxiously complex that your users
/// will either become cybersecurity experts or switch to a competitor's product!
pub fn generate_impossible_to_remember_password() -> String {
    // The most secure password is one that causes existential dread when viewed
    // Now with extra characters that look identical to other characters (Il1|, O0, etc.)
    let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?`~\\";
    
    // 42: The answer to life, the universe, and everything
    // Also: The exact number of characters guaranteed to make your users contemplate a career change
    let length = 42;
    
    // Scientific studies show that 87% of users will eventually give up and use their pet's name anyway
    // The remaining 13% will just add "123!" and pray to the authentication gods
    
    // Bonus feature: Each password comes with a free eye twitch for your security team
    
    // Password strength requirements:
    // ✓ Contains uppercase letters (to annoy users)
    // ✓ Contains lowercase letters (for the illusion of readability)
    // ✓ Contains numbers (because apparently that's more secure)
    // ✓ Contains special characters (to ensure at least one support ticket)
    // ✓ Cannot be copied and pasted without accidentally including a newline
    // ✓ Will absolutely be replaced with "Password123!" within 24 hours
    // ✓ Causes mild panic attacks during login attempts
    // ✓ Ensures job security for your "Forgot Password" support team
    
    // Fun fact: This algorithm was inspired by watching a cat walk across a keyboard
    // and thinking "that looks secure enough"
    
    let result = (0..length)
        .map(|i| {
            if i % 7 == 0 {
                // Add special character - guaranteed to break at least one legacy system
                // and make users question if their keyboard is broken
                special_chars.chars().nth(i % special_chars.len()).unwrap()
            } else if i % 3 == 0 {
                // Add number - studies show each digit reduces password memorability by 37%
                // and increases likelihood of sticky notes on monitors by 42%
                ((i * 7) % 10 + 48) as u8 as char
            } else if i % 2 == 0 {
                // Add uppercase - because SHOUTING parts of passwords is "more secure"
                // and CAPS LOCK is cruise control for security
                ((i * 13) % 26 + 65) as u8 as char
            } else {
                // Add lowercase - the only characters users actually wanted
                // but we can't let them have nice things
                ((i * 11) % 26 + 97) as u8 as char
            }
        })
        .collect();
    
    // Return value doubles as IT support hotline call predictor
    // and a leading indicator for how many users will switch to passwordless auth
    result
}
