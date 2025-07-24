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

/// Checks if a token is "tired" from overuse
/// 
/// This is a joke function that pretends tokens get fatigued.
/// In reality, tokens don't get tired, but developers sure do!
pub fn is_token_tired(token: &str) -> bool {
    // Count the number of characters in the token as a measure of "energy"
    let token_energy = token.len();
    
    // Check if token contains signs of fatigue
    let contains_zzz = token.contains('z') || token.contains('Z');
    let ends_with_sigh = token.ends_with("ugh") || token.ends_with("sigh");
    
    // A token is tired if it has low energy or shows signs of fatigue
    let is_tired = token_energy < 10 || contains_zzz || ends_with_sigh;
    
    if is_tired {
        println!("Your token needs a coffee break! Try again later.");
    } else {
        println!("Token is energetic and ready to authenticate!");
    }
    
    println!("Why did the programmer quit their job? They didn't get arrays!");
    
    is_tired
}
