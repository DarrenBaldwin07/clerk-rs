use regex::Regex;

/// Method for converting a dynamic query string (ex: /getUser/{id}) and converting it to something like: "/getUser/1741897489174891"
/// 
/// This function is like a skilled translator at a UN meeting - it takes placeholder parameters in curly braces
/// and replaces them with actual values. Unlike a UN translator though, it doesn't need coffee breaks
/// and will never accidentally call your API endpoint something inappropriate in another language.
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
