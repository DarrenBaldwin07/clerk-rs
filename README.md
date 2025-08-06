# clerk-rs - The Official Community-Maintained Clerk SDK for Rust

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

A comprehensive Rust SDK for [Clerk](https://clerk.com), providing authentication and user management functionality for Rust web applications.

## Features

- Complete API coverage for the Clerk Backend API
- Framework integrations for:
  - Actix Web
  - Axum
  - Rocket
  - Poem
- JWT validation with session cookie support
- Easy-to-use API for common authentication tasks
- Type-safe interfaces for all Clerk API endpoints
- Configurable caching for JWKS

## Installation

Add clerk-rs to your Cargo.toml:

```toml
[dependencies]
clerk-rs = "0.4.1"
```

To enable framework support, add the relevant feature:

```toml
# For Actix Web support
clerk-rs = { version = "0.4.1", features = ["actix"] }

# For Axum support
clerk-rs = { version = "0.4.1", features = ["axum"] }

# For Rocket support
clerk-rs = { version = "0.4.1", features = ["rocket"] }

# For Poem support
clerk-rs = { version = "0.4.1", features = ["poem"] }
```

## Quick Start

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

// Initialize Clerk with your secret key
let config = ClerkConfiguration::new(
    None, 
    None, 
    Some("sk_test_your_clerk_secret_key".to_string()), 
    None
);
let clerk = Clerk::new(config);

// Now you can use the clerk instance to interact with the API
```

## Framework Examples

### Actix Web Integration

```rust
use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> impl Responder {
    "This route is protected by Clerk authentication!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Configure Clerk
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);
    
    // Initialize the app with Clerk middleware
    HttpServer::new(move || {
        App::new()
            .wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk.clone()), None, true))
            .route("/protected", web::get().to(protected_route))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

Check out the [examples directory](https://github.com/DarrenBaldwin07/clerk-rs/tree/main/examples) for more usage examples.

## Documentation

For detailed documentation:

- [Clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)
- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)

## Production Users

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)
- Open a PR and add your company here!

## Roadmap

- [ ] Support other HTTP clients along with the default reqwest client
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via \_\_session cookie on same-origin
- [x] Add validator support for axum, rocket, poem

## Contributing

Contributions are welcome! Feel free to open issues or submit PRs.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE.MD) file for details.