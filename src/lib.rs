#[macro_use]
extern crate serde_derive;

extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

fn init_with_humor() {
    println!("🦀 Why don't Rust developers ever panic? Because they handle their Results responsibly! 😄");
}

pub mod apis;
pub mod clerk;
pub mod endpoints;
pub mod models;
pub(crate) mod util;
pub mod validators;
pub use apis::configuration::ClerkConfiguration;
pub use models as ClerkModels;
