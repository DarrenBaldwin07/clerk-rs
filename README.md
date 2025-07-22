# clerk-rs: The Official Community-Maintained Clerk SDK for Rust

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

A comprehensive Rust SDK for integrating with [Clerk](https://clerk.com/), the complete user management and authentication solution for modern applications.

## Features

- 🚀 **Full API Coverage**: Complete implementation of the Clerk Backend API
- 🔒 **Authentication**: Validate JWTs, sessions, and handle user authentication flows
- 🛡️ **Web Framework Integration**: Seamless middleware support for Actix, Axum, Rocket and Poem
- 💾 **Resource Management**: Create and manage users, organizations, sessions, and more
- 🧩 **Flexible Configuration**: Customize the SDK to fit your application's needs
- 🔍 **Type Safety**: Leverage Rust's type system for reliable API interactions

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.5.2"
```

## Framework Support

The SDK provides middleware for various Rust web frameworks:

- ✅ Actix Web
- ✅ Axum
- ✅ Rocket
- ✅ Poem

Enable the features you need:

```toml
[dependencies]
clerk-rs = { version = "0.5.2", features = ["actix", "axum", "rocket", "poem"] }
```

## Quick Start Examples

### Basic Setup

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

fn main() {
    // Create a Clerk client with your API key
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);
    
    // Now you can use the client to interact with Clerk API
}
```

### HTTP Requests

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Get list of users
    let res = client.get(ClerkGetEndpoint::GetUserList).await?;
    
    Ok(())
}
```

### Using API Methods

```rust
use tokio;
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

### Actix Web

```rust
use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
        let clerk = Clerk::new(config);

        App::new()
            .wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
            .route("/index", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### Axum

```rust
use axum::{routing::get, Router};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn index() -> &'static str {
    "Hello world!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/index", get(index))
        .layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), None, true));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
```

### More Examples

Check out the `/examples` directory for more detailed examples of using the SDK with different frameworks and use cases.

## Documentation

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [clerk-rs SDK API docs](https://docs.rs/clerk-rs)
- [SDK Documentation](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

## Roadmap

- [ ] Support other HTTP clients along with the default reqwest client (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via \_\_session cookie on same-origin

## Production Users

The following companies and projects are using clerk-rs in production:

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)

Using clerk-rs in production? Open a PR and add your company here!

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests.

> This SDK is updated frequently to keep up with any changes to the official Clerk API. If you see anything that needs updating or is not inline with the official Clerk API, please open an issue!