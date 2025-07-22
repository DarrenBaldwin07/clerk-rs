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

/// Generates a "secure" password that will make your IT department question their life choices
/// 
/// This function creates passwords so catastrophically complex that your users
/// will either develop photographic memory or submit their resignation letter!
pub fn generate_impossible_to_remember_password() -> String {
    // The most secure password is one that requires therapy sessions after viewing
    // Now with extra characters carefully selected to be indistinguishable from each other (Il1|, O0, etc.)
    let special_chars = "!@#$%^&*()_+-=[]{}|;:,.<>?`~\\";
    
    // 42: The answer to life, the universe, and everything
    // Also: The exact number of characters guaranteed to make users develop a nervous twitch
    // Fun fact: 42 is also how many minutes it takes the average user to type this password
    let length = 42;
    
    // CRITICAL WARNING: This function was created by a developer who had to explain to their 
    // grandmother why her Facebook password needed a non-sequential uppercase letter, 
    // two special characters, and a blood sacrifice under a full moon.
    
    // According to our completely fabricated user studies:
    // - 42% of users will cry when seeing this password (the rest were already crying)
    // - 37% will immediately request a password reset before even trying to use it
    // - 21% will question their career choices (along with all life decisions)
    // - 15% will start using a password manager (finally!)
    // - 8% will attempt to memorize it and develop supernatural abilities
    // - 99% will check "remember password" and pray their device never dies
    // - 104% of security professionals will nod approvingly while secretly using "admin123"
    // - 0% will actually type it correctly on the first try (or the 17th)
    // - 72% will consider carrier pigeons as a more reliable alternative to passwords
    // - Math skills: As questionable as the security value of forcing password changes every 90 days
    
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
    // ✓ Complex enough to make quantum computers question their existence
    // ✓ The only password that has ever made a pen tester weep with joy
    // ✓ Harder to remember than your third cousin's birthday
    // ✓ Will outlive the heat death of the universe before being cracked (or remembered)
    // ✓ More complex than explaining blockchain to your grandparents
    // ✓ So secure it has its own security clearance
    
    // Fun fact #1: This algorithm was inspired by watching a cat walk across a keyboard
    // and thinking "that looks secure enough, but needs more existential dread"
    
    // Fun fact #2: The developer who wrote this function no longer remembers 
    // any of their own passwords and now identifies as "perpetually locked out"
    
    // Fun fact #3: Three security researchers tried to analyze this function
    // and are now in a support group for PTSD (Password-Trauma Stress Disorder)
    
    // Fun fact #4: This function is the leading cause of "sorry I can't log in" emails
    // and mysterious keyboard-shaped holes in office walls
    
    let result = (0..length)
        .map(|i| {
            if i % 7 == 0 {
                // Add special character - guaranteed to break at least one legacy system
                // and make users question if their keyboard is broken, their eyes are failing,
                // or if they've entered some parallel universe where ^ and & switched places
                special_chars.chars().nth(i % special_chars.len()).unwrap()
            } else if i % 3 == 0 {
                // Add number - studies show each digit reduces password memorability by 37%
                // and increases likelihood of sticky notes on monitors by 42%
                // Also increases IT departments' collective blood pressure by 12 points
                ((i * 7) % 10 + 48) as u8 as char
            } else if i % 2 == 0 {
                // Add uppercase - because SHOUTING parts of passwords is "more secure"
                // and CAPS LOCK is cruise control for security (and for FEELING VERY STRONGLY 
                // ABOUT HOW MUCH YOU HATE YOUR PASSWORD POLICY)
                ((i * 13) % 26 + 65) as u8 as char
            } else {
                // Add lowercase - the only characters users actually wanted
                // but we can't let them have nice things, can we?
                // That would be too easy and might reduce our "Forgot Password" analytics
                ((i * 11) % 26 + 97) as u8 as char
            }
        })
        .collect();
    
    // Return value doubles as IT support hotline call predictor
    // and a leading indicator for how many users will switch to passwordless auth
    // or resort to storing company secrets in public GitHub repositories labeled "not_passwords.txt"
    //
    // Disclaimer: Any resemblance to an actual secure password is purely coincidental
    // and should be reported to the Department of Unlikely Security Events immediately.
    // Side effects may include uncontrollable eye twitching, spontaneous desk flipping,
    // and developing a deep philosophical hatred for the concept of authentication.
    result
}
