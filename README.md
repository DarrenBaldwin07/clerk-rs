# 🔐 clerk-rs

<div align="center">

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

**The official community-maintained Rust SDK for [Clerk](https://clerk.com)**

Powerful authentication and user management for Rust web applications
</div>

## 🚀 Features

- **Complete API Coverage** - Full support for Clerk's Backend API
- **Framework Integrations** - Built-in middleware for Actix, Axum, Rocket, and Poem
- **Type Safety** - Fully typed responses and requests for a seamless developer experience
- **JWT Verification** - Built-in JWT validation for secure authentication
- **Cookie-based Auth** - Support for session cookie validation on same-origin requests
- **Flexible Configuration** - Customize the client to fit your application's needs

## 📦 Installation

Add `clerk-rs` to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4.1"
```

To use framework-specific features:

```toml
[dependencies]
clerk-rs = { version = "0.4.1", features = ["actix"] } # For Actix support
# Other available features: "axum", "rocket", "poem"
```

## 🛠️ Quick Start

### Basic Client Setup

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

// Initialize with your Clerk secret key
let config = ClerkConfiguration::new(
    None, 
    None, 
    Some("sk_test_your_clerk_secret_key".to_string()), 
    None
);
let client = Clerk::new(config);
```

### Using HTTP Endpoints

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

// Get all users from your Clerk instance
let res = client.get(ClerkGetEndpoint::GetUserList).await?;

// Process the response
println!("User data: {:?}", res);
```

### Using API Methods

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::users_api::User};

// Get a specific user by ID
let user = User::get(&client, "user_id_here").await?;

// Update user information
let mut update_params = UpdateUserRequest::new();
update_params.first_name = Some("New Name".to_string());
User::update(&client, "user_id_here", Some(update_params)).await?;
```

## 🔒 Protecting Web Routes

### Actix Web

```rust
use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> impl Responder {
    "This route is protected by Clerk!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Setup Clerk client
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    // Create a server with middleware
    HttpServer::new(move || {
        App::new()
            .wrap(ClerkMiddleware::new(
                MemoryCacheJwksProvider::new(clerk.clone()), 
                None, 
                true
            ))
            .route("/protected", web::get().to(protected_route))
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

async fn protected_route() -> &'static str {
    "This route is protected by Clerk!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Setup Clerk client
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    // Create a router with middleware
    let app = Router::new()
        .route("/protected", get(protected_route))
        .layer(ClerkLayer::new(
            MemoryCacheJwksProvider::new(clerk), 
            None, 
            true
        ));

    // Start the server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
```

For more examples, check out the [examples directory](/examples).

## 📚 Documentation

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [clerk-rs SDK API docs](https://docs.rs/clerk-rs)

> This SDK is regularly updated to keep in sync with the official Clerk API. If you notice any discrepancies or missing features, please open an issue!

## 🚧 Roadmap

- [ ] Support additional HTTP clients (like hyper) alongside reqwest
- [ ] Support for both Tokio and async-std async runtimes
- [ ] Optional reqwest blocking client
- [x] Support authorization via `__session` cookie on same-origin
- [ ] Add validator support for other frameworks

## 🏢 Used In Production By

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)
- *Open a PR to add your company here!*

## 🤝 Contributing

Contributions are welcome! Feel free to open an issue or submit a pull request.

## 📝 License

This project is licensed under the MIT License - see the LICENSE.MD file for details.