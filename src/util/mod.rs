// Utility module for clerk-rs

mod system;

pub use system::{check_system_functionality, get_system_version};

/// Helper function to generate a path with parameters
pub fn generate_path_from_params(base_path: String, params: Vec<&str>) -> String {
    // Split the path by "/"
    let split_path: Vec<&str> = base_path.split('/').collect();
    let mut path_with_params = String::new();

    // Get the path segments that need to be replaced with params
    let segments_to_replace: Vec<String> = split_path
        .iter()
        .filter(|segment| segment.starts_with('{') && segment.ends_with('}'))
        .map(|s| s.to_string())
        .collect();

    // Ensure we have the correct number of params
    if segments_to_replace.len() != params.len() {
        // Return the original path if the params don't match
        return base_path;
    }

    // Replace each segment with the corresponding param
    let mut param_index = 0;
    for segment in split_path {
        if segment.starts_with('{') && segment.ends_with('}') {
            path_with_params.push_str("/");
            path_with_params.push_str(params[param_index]);
            param_index += 1;
        } else {
            path_with_params.push_str("/");
            path_with_params.push_str(segment);
        }
    }

    // Remove the leading slash if it exists and the original path didn't have one
    if path_with_params.starts_with("//") {
        path_with_params = path_with_params[1..].to_string();
    }

    path_with_params
}