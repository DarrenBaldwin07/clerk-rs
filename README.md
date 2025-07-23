# clerk-rs

The official community-maintained Rust SDK for the Clerk API. This library provides Rust bindings and integrations for Clerk's authentication and user management service.

[![Crates.io](https://img.shields.io/crates/v/clerk-rs.svg)](https://crates.io/crates/clerk-rs)
[![Documentation](https://docs.rs/clerk-rs/badge.svg)](https://docs.rs/clerk-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- Complete Rust bindings for the Clerk API
- Type-safe request and response handling
- JWT verification and session management
- Framework integrations for:
  - Actix Web
  - Axum
  - Rocket
  - Poem

## Installation

Add clerk-rs to your Cargo.toml:

```toml
[dependencies]
clerk-rs = "0.4.1"
```

### Framework Integrations

Enable specific framework integrations with feature flags:

```toml
# For Actix integration
clerk-rs = { version = "0.4.1", features = ["actix"] }

# For Axum integration
clerk-rs = { version = "0.4.1", features = ["axum"] }

# For Rocket integration
clerk-rs = { version = "0.4.1", features = ["rocket"] }

# For Poem integration
clerk-rs = { version = "0.4.1", features = ["poem"] }
```

## Basic Usage

### Making API Requests

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    let res = client.get(ClerkGetEndpoint::GetUserList).await?;

    Ok(())
}
```

### Using API Methods

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Create an email
    Email::create(&client, Some(your_clerk_email));

    Ok(())
}
```

## Framework Integration Examples

Check the `examples` directory for complete examples of integrating clerk-rs with various web frameworks.

## Production Users

clerk-rs is used in production by:

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE).

Copyright (c) 2023 Darren Baldwin