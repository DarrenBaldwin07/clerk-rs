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

/// Validates that a Clerk resource ID is not empty and follows the expected format
/// Clerk resource IDs typically follow specific patterns (e.g., "user_2KCnJpD3QptPj48Twy9z0qz2JAn")
pub fn validate_resource_id(id: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err("Resource ID cannot be empty".to_string());
    }

    // Most Clerk IDs follow a pattern of resource_type_<alphanumeric>
    // The validation below ensures the ID has a minimum format with a resource prefix and alphanumeric characters
    let id_regex = Regex::new(r"^[a-z]+_[A-Za-z0-9]+$").unwrap();
    if !id_regex.is_match(id) {
        return Err(format!("Invalid resource ID format: {}", id));
    }

    Ok(())
}
