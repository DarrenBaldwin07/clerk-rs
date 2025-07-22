# clerk-rs: The Official Community-Maintained Clerk SDK for Rust

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A comprehensive Rust SDK for integrating with [Clerk](https://clerk.com/), the complete user management and authentication solution for modern applications.

## Table of Contents

- [Features](#features)
- [Installation](#installation)
- [Framework Support](#framework-support)
- [Usage](#usage)
  - [Basic Setup](#basic-setup)
  - [Authentication](#authentication)
  - [User Management](#user-management)
  - [Organizations](#organizations)
- [Framework Integration Examples](#framework-integration-examples)
  - [Actix Web](#actix-web)
  - [Axum](#axum)
  - [Rocket](#rocket)
  - [Poem](#poem)
- [Advanced Usage](#advanced-usage)
  - [Customizing Request Configuration](#customizing-request-configuration)
  - [Error Handling](#error-handling)
  - [Webhook Verification](#webhook-verification)
- [Documentation](#documentation)
- [Roadmap](#roadmap)
- [Production Users](#production-users)
- [Contributing](#contributing)
- [License](#license)

## Features

- 🚀 **Full API Coverage**: Complete implementation of the Clerk Backend API
- 🔒 **Authentication**: Validate JWTs, sessions, and handle user authentication flows
- 🛡️ **Web Framework Integration**: Seamless middleware support for Actix, Axum, Rocket and Poem
- 💾 **Resource Management**: Create and manage users, organizations, sessions, and more
- 🧩 **Flexible Configuration**: Customize the SDK to fit your application's needs
- 🔍 **Type Safety**: Leverage Rust's type system for reliable API interactions
- 🧪 **Well-Tested**: Comprehensive test suite ensures reliability

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.5.2"
```

For a specific framework integration, enable the corresponding feature:

```toml
[dependencies]
clerk-rs = { version = "0.5.2", features = ["actix"] }
```

## Framework Support

The SDK provides middleware for various Rust web frameworks:

| Framework | Feature Flag | Status |
|-----------|--------------|--------|
| Actix Web | `actix`      | ✅     |
| Axum      | `axum`       | ✅     |
| Rocket    | `rocket`     | ✅     |
| Poem      | `poem`       | ✅     |

Enable multiple frameworks as needed:

```toml
[dependencies]
clerk-rs = { version = "0.5.2", features = ["actix", "axum", "rocket", "poem"] }
```

## Usage

### Basic Setup

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

fn main() {
    // Create a Clerk client with your API key
    let config = ClerkConfiguration::new(
        None,                            // Frontend API URL (optional)
        None,                            // API URL (optional)
        Some("your_secret_key".to_string()), // Secret key
        None,                            // Custom JWT key (optional)
    );
    let clerk = Clerk::new(config);
    
    // Now you can use the client to interact with Clerk API
}
```

### Authentication

Verify a session token:

```rust
use clerk_rs::{
    clerk::Clerk,
    validators::session::validate_session_token,
    ClerkConfiguration,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Validate a session token
    let session = validate_session_token(&client, "sess_token").await?;
    println!("Session belongs to user: {}", session.user_id);
    
    Ok(())
}
```

### User Management

```rust
use clerk_rs::{
    clerk::Clerk,
    apis::users_api::User,
    ClerkConfiguration,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Create a new user
    let user_data = json!({
        "email_address": ["user@example.com"],
        "password": "SecurePassword123",
        "first_name": "John",
        "last_name": "Doe"
    });
    
    let new_user = User::create(&client, Some(user_data)).await?;
    println!("Created user with ID: {}", new_user.id);
    
    // Get user by ID
    let user = User::get(&client, &new_user.id).await?;
    
    // Update user
    let update_data = json!({
        "first_name": "Jane"
    });
    User::update(&client, &user.id, Some(update_data)).await?;
    
    Ok(())
}
```

### Organizations

```rust
use clerk_rs::{
    clerk::Clerk,
    apis::organizations_api::Organization,
    ClerkConfiguration,
};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Create a new organization
    let org_data = json!({
        "name": "My Organization",
        "slug": "my-org"
    });
    
    let new_org = Organization::create(&client, Some(org_data)).await?;
    println!("Created organization with ID: {}", new_org.id);
    
    // Add a user to the organization
    Organization::add_member(&client, &new_org.id, "user_id", "admin").await?;
    
    Ok(())
}
```

## Framework Integration Examples

### Actix Web

```rust
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
    models::auth::ClerkUserAuth,
};

// Protected route that requires authentication
async fn protected_route(user_auth: ClerkUserAuth) -> impl Responder {
    HttpResponse::Ok().json(format!("Hello, {}!", user_auth.user_id))
}

// Public route that doesn't require authentication
async fn public_route() -> impl Responder {
    HttpResponse::Ok().body("This is a public route")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
        let clerk = Clerk::new(config);

        App::new()
            .wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
            .service(
                web::scope("/api")
                    .route("/protected", web::get().to(protected_route))
                    .route("/public", web::get().to(public_route))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### Axum

```rust
use axum::{
    routing::get,
    Router, extract::State,
    response::{IntoResponse, Json},
};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
    models::auth::ClerkUserAuth,
};
use std::sync::Arc;

// Protected route that requires authentication
async fn protected_route(auth: ClerkUserAuth) -> impl IntoResponse {
    Json(format!("Hello, {}!", auth.user_id))
}

// Public route that doesn't require authentication
async fn public_route() -> impl IntoResponse {
    "This is a public route"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/api/protected", get(protected_route))
        .route("/api/public", get(public_route))
        .layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), None, true));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
```

### Rocket

See the `/examples` directory for more detailed examples of using the SDK with Rocket.

### Poem

See the `/examples` directory for more detailed examples of using the SDK with Poem.

## Advanced Usage

### Customizing Request Configuration

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};
use reqwest::ClientBuilder;
use std::time::Duration;

fn main() {
    // Create a custom reqwest client with specific configuration
    let client_builder = ClientBuilder::new()
        .timeout(Duration::from_secs(30))
        .user_agent("my-application/1.0");
    
    // Configure Clerk with custom HTTP client
    let mut config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    config.set_client_builder(Some(client_builder));
    
    let clerk = Clerk::new(config);
}
```

### Error Handling

```rust
use clerk_rs::{
    clerk::Clerk,
    apis::users_api::User,
    error::ClerkError,
    ClerkConfiguration,
};

#[tokio::main]
async fn main() {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Handle possible errors
    match User::get(&client, "nonexistent_user_id").await {
        Ok(user) => {
            println!("Found user: {}", user.id);
        },
        Err(ClerkError::NotFound) => {
            println!("User not found");
        },
        Err(ClerkError::Unauthorized) => {
            println!("Invalid API key or insufficient permissions");
        },
        Err(e) => {
            println!("An error occurred: {:?}", e);
        }
    }
}
```

### Webhook Verification

```rust
use clerk_rs::webhooks::verify_webhook;

fn handle_webhook(payload: &[u8], clerk_webhook_signature: &str, webhook_secret: &str) -> Result<(), String> {
    // Verify the webhook signature
    if verify_webhook(payload, clerk_webhook_signature, webhook_secret) {
        // Process the verified webhook
        println!("Webhook verified successfully");
        Ok(())
    } else {
        Err("Invalid webhook signature".to_string())
    }
}
```

## Documentation

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [clerk-rs SDK API docs](https://docs.rs/clerk-rs)
- [SDK Documentation](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)
- [Examples directory](https://github.com/DarrenBaldwin07/clerk-rs/tree/main/examples)

## Roadmap

- [ ] Support other HTTP clients along with the default reqwest client (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via \_\_session cookie on same-origin
- [ ] Improved documentation and examples
- [ ] OAuth integration helpers

## Production Users

The following companies and projects are using clerk-rs in production:

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)

Using clerk-rs in production? Open a PR and add your company here!

## Contributing

Contributions are welcome! Here's how you can contribute:

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

> This SDK is updated frequently to keep up with any changes to the official Clerk API. If you see anything that needs updating or is not inline with the official Clerk API, please open an issue!

## License

This project is licensed under the MIT License - see the LICENSE file for details.