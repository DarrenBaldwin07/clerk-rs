use regex::Regex;

/// Validates that an ID parameter is non-empty and only contains valid characters
/// Returns an error if the ID is invalid
pub fn validate_id(id: &str, id_type: &str) -> Result<(), String> {
    if id.is_empty() {
        return Err(format!("{} ID cannot be empty", id_type));
    }
    
    // Clerk IDs typically follow a format like "saml_conn_123abc..."
    // This regex allows alphanumeric characters, underscores, and hyphens
    let id_regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    
    if !id_regex.is_match(id) {
        return Err(format!(
            "Invalid {} ID format: '{}'. ID should only contain alphanumeric characters, underscores, and hyphens.",
            id_type, id
        ));
    }
    
    Ok(())
}

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
