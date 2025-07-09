[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# clerk-rs: The Official Community-Maintained Clerk SDK for Rust

A Rust SDK for [Clerk](https://clerk.com/) authentication with support for Actix, Axum, Rocket, and Poem.

## Installation

```toml
# Basic usage
clerk-rs = "0.4.1"

# With framework support
clerk-rs = { version = "0.4.1", features = ["actix"] } # or "axum", "rocket", "poem"
```

## Documentation
- [API Docs](https://docs.rs/clerk-rs)
- [Clerk Backend API](https://clerk.com/docs/reference/backend-api)

## Usage Example

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);
    
    // Get user list
    let res = client.get(ClerkGetEndpoint::GetUserList).await?;
    
    Ok(())
}
```

More examples for Actix, Axum, Rocket, and Poem in the `/examples` directory.

## Features
- JWT validation with memory caching
- Session cookie support
- Framework support: Actix, Axum, Rocket, Poem

## Production Users
[Tembo](https://tembo.io), [Rezon](https://rezon.ai), [Gitar](https://gitar.co), [Have I Been Squatted](https://haveibeensquatted.com)

## License
MIT