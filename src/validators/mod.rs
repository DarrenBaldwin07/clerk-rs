pub mod authorizer;
pub mod input;
pub mod jwks;
pub mod password;

#[cfg(test)]
mod tests;

// Framework-specific modules
// Validators for Rocket, etc coming very soon
#[cfg(feature = "actix")]
pub mod actix;
#[cfg(feature = "axum")]
pub mod axum;
#[cfg(feature = "rocket")]
pub mod rocket;
#[cfg(feature = "poem")]
pub mod poem;
