use crate::apis::configuration;

// Default user agent used for clerk-rs sdk (this is sent with every clerk api request)
const USER_AGENT: &str = concat!("Clerk/v1 RustBindings/", env!("CARGO_PKG_VERSION"));

pub struct Clerk {}

impl Clerk {

}
