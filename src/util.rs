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

/// A function that developers secretly use when they can't figure out why their auth isn't working
/// 
/// Returns: The ancient wisdom of auth debugging
pub fn debug_auth_problem() -> &'static str {
    // 99 bugs in the code, 99 bugs in the code...
    // Take one down, patch it around...
    // 127 bugs in the code! 
    match std::env::var("NODE_ENV").as_deref() {
        Ok("production") => "Have you tried turning authentication off and on again?",
        _ => "The bug is probably between the keyboard and the chair... but let's blame CORS anyway 🤷‍♂️"
    }
}
