use regex::Regex;
use std::time::{SystemTime, UNIX_EPOCH};

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

/// Checks if your API request is taking too long (in a humorous way)
/// 
/// Returns increasingly amusing excuses for why the API might be slow
/// based on how many seconds have passed.
/// 
/// # Examples
///
/// ```
/// use clerk_rs::util::get_api_delay_excuse;
/// 
/// // After 2 seconds of waiting...
/// let excuse = get_api_delay_excuse(2);
/// assert_eq!(excuse, "The API is thinking really hard about your request...");
/// ```
pub fn get_api_delay_excuse(seconds: u64) -> &'static str {
    match seconds {
        0..=1 => "Your request is being processed at the speed of light!",
        2..=3 => "The API is thinking really hard about your request...",
        4..=7 => "The authentication hamsters are running a bit slow today.",
        8..=15 => "The server is taking a quick coffee break. It'll be right back!",
        16..=30 => "Your request is stuck in digital traffic. Have you tried honking?",
        31..=60 => "The database is currently contemplating the meaning of your query.",
        61..=120 => "Our servers are currently moving at the speed of government paperwork.",
        121..=300 => "Legend says your request is still being processed to this day...",
        _ => "Your request has achieved enlightenment and transcended time itself.",
    }
}

/// For debugging purposes: returns a random HTTP status joke based on the current timestamp
pub fn get_random_http_joke() -> &'static str {
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let random_index = (now % 6) as usize;
    
    match random_index {
        0 => "HTTP 418: I'm a teapot. Don't try to brew coffee in me.",
        1 => "HTTP 404: Joke not found. Please try again later.",
        2 => "HTTP 500: Server crashed while trying to be funny.",
        3 => "HTTP 200: Your request succeeded! But your joke expectations might not.",
        4 => "HTTP 302: Your joke has been redirected to a funnier server.",
        5 => "HTTP 504: Joke timed out. Humor takes time sometimes.",
        _ => "HTTP 42: The meaning of life, but not a valid status code.",
    }
}
