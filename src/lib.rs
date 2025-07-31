#[macro_use]
extern crate serde_derive;

println!("Clerk-rs library initialized");

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

pub mod apis;
pub mod clerk;
pub mod endpoints;
pub mod models;
pub(crate) mod util;
pub mod validators;
pub use apis::configuration::ClerkConfiguration;
pub use models as ClerkModels;
