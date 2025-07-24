# clerk-rs 🔐

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

The **official community-maintained Clerk SDK for Rust** 🦀

A comprehensive Rust SDK for integrating with [Clerk](https://clerk.com), providing authentication and user management capabilities for your Rust applications. This SDK supports multiple web frameworks and offers both high-level APIs and low-level HTTP client access.

## 🚀 Features

- **Full Clerk API Coverage**: Complete access to Clerk's Backend API
- **Multiple Framework Support**: Built-in middleware for Actix Web, Axum, Rocket, and Poem
- **JWT Validation**: Secure JWT token validation with JWKS support
- **Session Management**: Cookie-based session validation
- **Type Safety**: Fully typed API responses and requests
- **Async/Await**: Built on modern async Rust patterns
- **Flexible HTTP Client**: Uses reqwest with customizable TLS backends

## 📦 Installation

Add `clerk-rs` to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4.1"

# Optional: Enable framework-specific features
clerk-rs = { version = "0.4.1", features = ["actix"] }
clerk-rs = { version = "0.4.1", features = ["axum"] }
clerk-rs = { version = "0.4.1", features = ["rocket"] }
clerk-rs = { version = "0.4.1", features = ["poem"] }
```

### Available Features

- `actix` - Actix Web middleware support
- `axum` - Axum framework support  
- `rocket` - Rocket framework support
- `poem` - Poem framework support
- `native-tls` - Use native TLS implementation
- `rustls-tls` - Use rustls TLS implementation (default)

## 🔧 Quick Start

### Basic Setup

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Clerk client
    let config = ClerkConfiguration::new(
        None,                                    // domain
        None,                                    // api_url  
        Some("sk_test_your_secret_key".to_string()), // secret_key
        None,                                    // api_version
    );
    let client = Clerk::new(config);
    
    // Your application logic here
    Ok(())
}
```

### Environment Variables

You can also configure Clerk using environment variables:

```bash
export CLERK_SECRET_KEY="sk_test_your_secret_key"
export CLERK_DOMAIN="your-domain.clerk.accounts.dev"
```

## 📚 Usage Examples

### Making API Calls

#### Using HTTP Endpoints

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Get list of users
    let users = client.get(ClerkGetEndpoint::GetUserList).await?;
    println!("Users: {:?}", users);

    Ok(())
}
```

#### Using High-Level API Methods

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Create an email
    let email_result = Email::create(&client, Some("user@example.com")).await?;
    
    Ok(())
}
```

### Framework Integration

#### Actix Web

Protect your Actix Web routes with Clerk authentication:

```rust
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_handler() -> impl Responder {
    HttpResponse::Ok().json("Hello, authenticated user!")
}

async fn public_handler() -> impl Responder {
    HttpResponse::Ok().json("This endpoint is public")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("your_secret_key".to_string()), 
        None
    );
    let clerk = Clerk::new(config);

    HttpServer::new(move || {
        App::new()
            .wrap(ClerkMiddleware::new(
                MemoryCacheJwksProvider::new(clerk.clone()), 
                None, 
                true // validate_session_cookie
            ))
            .route("/protected", web::get().to(protected_handler))
            .route("/public", web::get().to(public_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

#### Axum

Secure your Axum application with Clerk:

```rust
use axum::{routing::get, Router, response::Json};
use serde_json::{json, Value};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_handler() -> Json<Value> {
    Json(json!({"message": "Hello, authenticated user!"}))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("your_secret_key".to_string()), 
        None
    );
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/protected", get(protected_handler))
        .layer(ClerkLayer::new(
            MemoryCacheJwksProvider::new(clerk), 
            None, 
            true // validate_session_cookie
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

#### Rocket

Integrate Clerk with Rocket using guards:

```rust
use clerk_rs::{
    clerk::Clerk,
    validators::{
        jwks::MemoryCacheJwksProvider,
        rocket::{ClerkGuard, ClerkGuardConfig},
    },
    ClerkConfiguration,
};
use rocket::{get, launch, routes, serde::json::Json};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    message: String,
}

#[get("/protected")]
fn protected_handler(_jwt: ClerkGuard<MemoryCacheJwksProvider>) -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "Hello, authenticated user!".to_string(),
    })
}

#[launch]
fn rocket() -> _ {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("your_secret_key".to_string()), 
        None
    );
    let clerk = Clerk::new(config);
    let clerk_config = ClerkGuardConfig::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true, // validate_session_cookie
    );

    rocket::build()
        .mount("/", routes![protected_handler])
        .manage(clerk_config)
}
```

#### Poem

Use Clerk with the Poem web framework:

```rust
use clerk_rs::{
    clerk::Clerk,
    validators::{jwks::MemoryCacheJwksProvider, poem::ClerkPoemMiddleware},
    ClerkConfiguration,
};
use poem::{
    get, handler, listener::TcpListener, web::Path, 
    EndpointExt, Route, Server, Result as PoemResult
};

#[handler]
fn protected_handler(Path(name): Path<String>) -> PoemResult<String> {
    Ok(format!("Hello, authenticated user: {}", name))
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = ClerkConfiguration::new(
        None,
        None,
        Some("your_secret_key".to_string()),
        None,
    );
    let clerk = Clerk::new(config);
    
    let clerk_middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        true, // validate_session_cookie
        Some(vec!["/health".to_string()]), // excluded routes
    );

    let app = Route::new()
        .at("/user/:name", get(protected_handler))
        .with(clerk_middleware);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

## 🔍 API Reference

For detailed API documentation, please refer to:

- **[Official Clerk Backend API Documentation](https://clerk.com/docs/reference/backend-api)** - Complete API reference
- **[Clerk-rs SDK Documentation](https://docs.rs/clerk-rs)** - Rust-specific documentation
- **[SDK API Documentation](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)** - Additional SDK details

## 📖 Examples

Check out the [`/examples`](./examples) directory for complete working examples:

- **[`http.rs`](./examples/http.rs)** - Basic HTTP client usage
- **[`api.rs`](./examples/api.rs)** - High-level API methods
- **[`actix.rs`](./examples/actix.rs)** - Actix Web integration
- **[`axum.rs`](./examples/axum.rs)** - Axum framework integration  
- **[`rocket.rs`](./examples/rocket.rs)** - Rocket framework integration

Run an example:

```bash
# Basic HTTP example
cargo run --example http

# Actix Web example (requires actix feature)
cargo run --example actix --features actix

# Axum example (requires axum feature)  
cargo run --example axum --features axum
```

## 🛣️ Roadmap

- [ ] **HTTP Client Flexibility**: Support for additional HTTP clients beyond reqwest (hyper, etc.)
- [ ] **Runtime Support**: Tokio and async-std runtime compatibility for hyper clients
- [ ] **Blocking Client**: Optional reqwest blocking client support
- [x] **Session Cookie Support**: Authorization via `__session` cookie on same-origin requests
- [ ] **Framework Expansion**: Additional validator support for warp and other frameworks
- [ ] **Caching Improvements**: Enhanced JWKS caching strategies
- [ ] **Error Handling**: Improved error types and handling

## 🏢 Production Users

Companies and projects using `clerk-rs` in production:

- **[Tembo](https://tembo.io)** - Postgres platform
- **[Rezon](https://rezon.ai)** - AI-powered solutions
- **[Gitar](https://gitar.co)** - Development tools
- **[Have I Been Squatted](https://haveibeensquatted.com)** - Domain security

*Using clerk-rs in production? [Open a PR](https://github.com/DarrenBaldwin07/clerk-rs/pulls) and add your company!*

## 🤝 Contributing

We welcome contributions! This is a community-maintained project, and we appreciate:

- 🐛 **Bug Reports**: Found an issue? Please open an issue with details
- 🚀 **Feature Requests**: Have an idea? Let's discuss it in an issue
- 📝 **Documentation**: Help improve our docs and examples
- 🔧 **Code Contributions**: Submit PRs for bug fixes and new features

### Development Setup

```bash
# Clone the repository
git clone https://github.com/DarrenBaldwin07/clerk-rs.git
cd clerk-rs

# Run tests
cargo test

# Run examples
cargo run --example http

# Check formatting
cargo fmt --check

# Run clippy
cargo clippy
```

## 📄 License

This project is licensed under the [MIT License](LICENSE.MD).

## 🔄 Updates

This SDK is updated frequently to stay in sync with the official Clerk API. If you notice anything that needs updating or isn't aligned with the official Clerk API, please [open an issue](https://github.com/DarrenBaldwin07/clerk-rs/issues)!

---

<div align="center">

**Built with ❤️ by the Rust community**

[Documentation](https://docs.rs/clerk-rs) • [Examples](./examples) • [Issues](https://github.com/DarrenBaldwin07/clerk-rs/issues) • [Clerk.com](https://clerk.com)

</div>

