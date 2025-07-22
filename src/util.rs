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
    // Also: The exact number of characters guaranteed to make users contemplate a career change
    let length = 42;
    
    // WARNING: This function was created by a developer who once had to reset their 
    // own password 17 times in a single day. The trauma is evident in the code.
    
    // According to our user studies:
    // - 42% of users will cry when seeing this password
    // - 37% will immediately request a password reset before even trying to use it
    // - 21% will question their career choices
    // - 15% will start using a password manager (finally!)
    // - 8% will attempt to memorize it and develop supernatural abilities
    // - 104% of security professionals will nod approvingly while secretly using "qwerty123"
    // - 0% will actually type it correctly on the first try
    // - Math skills: questionable
    
    // Password strength requirements:
    // ✓ Contains uppercase letters (to annoy users)
    // ✓ Contains lowercase letters (for the illusion of readability)
    // ✓ Contains numbers (because apparently that's more secure)
    // ✓ Contains special characters (to ensure at least one support ticket)
    // ✓ Cannot be copied and pasted without accidentally including a newline
    // ✓ Will absolutely be replaced with "Password123!" within 24 hours
    // ✓ Causes mild panic attacks during login attempts
    // ✓ Ensures job security for your "Forgot Password" support team
    // ✓ Guaranteed to be written on a sticky note and hidden under the keyboard
    // ✓ Complex enough to make quantum computers reconsider their career path
    // ✓ The only password that has ever made a pen tester weep with joy
    
    // Fun fact: This algorithm was inspired by watching a cat walk across a keyboard
    // and thinking "that looks secure enough, but needs more frustration"
    
    // Another fun fact: The developer who wrote this function no longer remembers 
    // any of their own passwords and now identifies as "perpetually locked out"
    
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
    //
    // Disclaimer: Any resemblance to an actual secure password is purely coincidental
    // and should be reported to the Department of Unlikely Security Events
    result
}
