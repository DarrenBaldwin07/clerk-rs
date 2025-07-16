# clerk-rs

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE)

The official community-maintained Rust SDK for [Clerk](https://clerk.com), a powerful user authentication and identity platform.

## Features

- 🚀 Complete Rust bindings for the Clerk Backend API
- 🔒 JWT validation middleware for popular Rust web frameworks
- 🧩 Optional features for framework-specific integrations
- 🔄 Fully async with Tokio runtime support
- 📦 Rich type definitions for all Clerk API resources

## Supported Frameworks

- **Actix Web** - Complete middleware for protecting routes
- **Axum** - Layer-based authentication
- **Rocket** - Guard-based protection
- **Poem** - Middleware integration

## Installation

Add clerk-rs to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4.1"
```

### With Framework Support

Enable the framework-specific features you need:

```toml
[dependencies]
clerk-rs = { version = "0.4.1", features = ["actix", "axum", "rocket", "poem"] }
```

## Usage Examples

### Basic API Client

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::users_api::User};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Clerk client
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Get all users
    let users = User::get_user_list(&client, None, None).await?;
    println!("Found {} users", users.len());

    Ok(())
}
```

### Protecting Routes with Actix Web

```rust
use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn index() -> impl Responder {
    "Hello, authenticated user!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    HttpServer::new(|| {
        App::new()
            .wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
            .route("/protected", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### Protecting Routes with Axum

```rust
use axum::{routing::get, Router};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> &'static str {
    "Hello, authenticated user!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/protected", get(protected_route))
        .layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), None, true));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
```

### Protecting Routes with Rocket

```rust
use clerk_rs::{
    clerk::Clerk,
    validators::{
        jwks::MemoryCacheJwksProvider,
        rocket::{ClerkGuard, ClerkGuardConfig},
    },
    ClerkConfiguration,
};
use rocket::{get, launch, routes};

#[get("/protected")]
fn protected(jwt: ClerkGuard<MemoryCacheJwksProvider>) -> &'static str {
    "Hello, authenticated user!"
}

#[launch]
fn rocket() -> _ {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);
    let clerk_config = ClerkGuardConfig::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true, // validate_session_cookie
    );

    rocket::build().mount("/", routes![protected]).manage(clerk_config)
}
```

### Protecting Routes with Poem

```rust
use clerk_rs::{
    clerk::Clerk,
    validators::{jwks::MemoryCacheJwksProvider, poem::ClerkPoemMiddleware},
    ClerkConfiguration,
};
use poem::{get, handler, listener::TcpListener, EndpointExt, Route, Server};

#[handler]
fn protected() -> String {
    "Hello, authenticated user!".to_string()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let clerk = Clerk::new(ClerkConfiguration::new(
        None,
        None,
        Some("your_secret_key".to_owned()),
        None,
    ));

    let clerk_poem_middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk.clone()),
        true,
        Some(vec!["/public".to_owned()]),
    );

    let app = Route::new()
        .at("/protected", get(protected))
        .with(clerk_poem_middleware);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

## More Examples

Check the `/examples` directory for more detailed usage examples.

## Documentation

- [Clerk Backend API Documentation](https://clerk.com/docs/reference/backend-api)
- [clerk-rs API Documentation](https://docs.rs/clerk-rs)
- [SDK API Guide](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

## Configuration Options

The `ClerkConfiguration` struct allows customizing your Clerk client:

```rust
let config = ClerkConfiguration::new(
    None,                      // Base URL (optional)
    None,                      // User agent (optional)
    Some("sk_test_key".to_string()), // Secret key (required)
    None,                      // Timeout (optional)
);
```

## Feature Flags

- `rustls-tls` (default) - Uses rustls for TLS
- `native-tls` - Uses native TLS implementation
- `actix` - Enables Actix Web integration
- `axum` - Enables Axum integration
- `rocket` - Enables Rocket integration
- `poem` - Enables Poem integration

## Roadmap

- [ ] Support for additional HTTP clients beyond reqwest
- [ ] Support for different async runtimes
- [ ] Optional blocking client
- [x] Support authorization via __session cookie on same-origin
- [ ] Additional validator support for other frameworks

## Production Users

The following companies use clerk-rs in production:

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)

Open a PR to add your company here!

## Contributing

Contributions are welcome! This SDK is updated frequently to keep up with changes to the Clerk API. If you find anything that needs updating or is not aligned with the official Clerk API, please open an issue or submit a pull request.

## License

This project is licensed under the MIT License.