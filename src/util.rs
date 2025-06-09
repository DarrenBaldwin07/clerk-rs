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

/// Validates a template ID to ensure it's not empty and matches the expected format
/// 
/// Template IDs in Clerk typically follow a specific format (e.g., 'tmpl_abc123').
/// This function checks if the provided ID is valid.
/// 
/// # Arguments
/// 
/// * `template_id` - The template ID to validate
/// 
/// # Returns
/// 
/// * `Ok(())` if the template ID is valid
/// * `Err(String)` with an error message if the template ID is invalid
pub fn validate_template_id(template_id: &str) -> Result<(), String> {
    if template_id.is_empty() {
        return Err("Template ID cannot be empty".to_string());
    }

    // Clerk template IDs typically follow the format 'tmpl_' followed by alphanumeric characters
    let template_id_regex = Regex::new(r"^tmpl_[a-zA-Z0-9]+$").unwrap();
    if !template_id_regex.is_match(template_id) {
        return Err(format!("Invalid template ID format: {}", template_id));
    }

    Ok(())
}
