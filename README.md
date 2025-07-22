<div align="center">

# 🔐 clerk-rs

**The official community-maintained Clerk SDK for Rust**

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

*Seamlessly integrate Clerk authentication and user management into your Rust applications*

</div>

## 📖 Overview

Clerk-rs is a powerful, type-safe Rust SDK for [Clerk](https://clerk.com) - the authentication and user management platform built for the modern web. Integrate Clerk's powerful authentication and user management features into your Rust applications with ease.

### Key Features

- **Complete API Coverage** - Access all Clerk backend API features with type-safe Rust interfaces
- **Web Framework Integration** - First-class support for popular Rust web frameworks (Axum, Actix, Rocket, Poem)
- **Type Safety** - Leverage Rust's strong type system for safer code and better developer experience
- **JWT Validation** - Built-in JWT validation with middleware for popular web frameworks
- **Fully Async** - Designed with async/await for efficient non-blocking operations

### Documentation

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [Clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

> **Active Development**: This SDK is updated frequently to keep up with any changes to the Clerk API. If you see anything that needs updating or is not inline with the official Clerk API, please open an issue!

## 💻 Getting Started

Add `clerk-rs` to your project's dependencies:

```toml
# Cargo.toml
[dependencies]
clerk-rs = "0.4.1"
```

Optionally enable web framework integrations:

```toml
# With Axum support
clerk-rs = { version = "0.4.1", features = ["axum"] }

# With Actix support
clerk-rs = { version = "0.4.1", features = ["actix"] }

# With Rocket support
clerk-rs = { version = "0.4.1", features = ["rocket"] }

# With Poem support
clerk-rs = { version = "0.4.1", features = ["poem"] }
```

## 📚 Examples

> **Note:** Check out complete examples in the `/examples` directory

### Basic Usage

#### Direct HTTP Request

Make a direct HTTP request to a Clerk endpoint:

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Clerk with your secret key
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Fetch user list
    let res = client.get(ClerkGetEndpoint::GetUserList).await?;
    
    // Process response...

    Ok(())
}
```

#### Using Clerk-rs Methods

Use type-safe methods for Clerk operations:

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Clerk with your secret key
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Create a new email
    Email::create(&client, Some(your_clerk_email));

    Ok(())
}
```

## 🔒 Web Framework Integration

Clerk-rs provides first-class support for popular Rust web frameworks.

### Actix-web Integration

With the `actix` feature enabled:

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
        // Initialize Clerk with your secret key
        let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
        let clerk = Clerk::new(config);

        // Create the app with Clerk middleware
        App::new()
            .wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
            .route("/index", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### Axum Integration

With the `axum` feature enabled:

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
    // Initialize Clerk with your secret key
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    // Create the app with Clerk layer
    let app = Router::new()
        .route("/index", get(index))
        .layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), None, true));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
```

### Rocket Integration

With the `rocket` feature enabled:

```rust
use clerk_rs::{
	clerk::Clerk,
	validators::{
		jwks::MemoryCacheJwksProvider,
		rocket::{ClerkGuard, ClerkGuardConfig},
	},
	ClerkConfiguration,
};
use rocket::{
	get, launch, routes,
	serde::{Deserialize, Serialize},
};

#[derive(Serialize, Deserialize)]
struct Message {
	content: String,
}

#[get("/")]
fn index(jwt: ClerkGuard<MemoryCacheJwksProvider>) -> &'static str {
	"Hello world!"
}

#[launch]
fn rocket() -> _ {
    // Initialize Clerk with your secret key
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);
    
    // Create ClerkGuardConfig for Rocket
    let clerk_config = ClerkGuardConfig::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true, // validate_session_cookie
    );

	rocket::build().mount("/", routes![index]).manage(clerk_config)
}

```

### Poem Integration

With the `poem` feature enabled and poem v3 installed:
```rust
use clerk_rs::{
    clerk::Clerk,
    validators::{jwks::MemoryCacheJwksProvider, poem::ClerkPoemMiddleware},
    ClerkConfiguration,
};
use poem::{get, handler, listener::TcpListener, web::Path, EndpointExt, Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialize Clerk with your secret key
    let clerk = Clerk::new(ClerkConfiguration::new(
        None,
        None,
        Some("your_secret_key".to_string()),
        None,
    ));
    
    // Initialize middleware
    let clerk_poem_middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk.clone()),
        true,
        // Routes to exclude from auth verification
        Some(vec!["/some/route/to/exclude".to_owned()]),
    );

    // Create the app with Clerk middleware
    let app = Route::new()
        .at("/hello/:name", get(hello))
        .with(clerk_poem_middleware);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

> **Accessing JWT Data**: In Poem, the JWT can be accessed using `Data<&ClerkJwt>` (or `req.data::<ClerkJwt>()`)

## 📍 Roadmap

We're constantly working to improve clerk-rs. Here's what's coming next:

- [ ] **HTTP Client Flexibility** - Support alternative HTTP clients alongside reqwest (like hyper)
- [ ] **Runtime Support** - Add support for Tokio and async-std async runtimes for hyper clients
- [ ] **Blocking Client** - Optional reqwest blocking client for synchronous operations
- [x] **Cookie Auth** - Support authorization via \_\_session cookie on same-origin
- [ ] **Additional Framework Support** - Expand validator support for additional frameworks (warp)

Interested in contributing? Check out our [issues](https://github.com/DarrenBaldwin07/clerk-rs/issues) or open a PR!

## 🌐 Production Users

Clerk-rs is trusted in production by innovative companies and projects:

- [**Tembo**](https://tembo.io) - Enterprise PostgreSQL platform
- [**Rezon**](https://rezon.ai) - AI-powered creative optimization
- [**Gitar**](https://gitar.co) - Modern Git-based collaboration platform
- [**Have I Been Squatted**](https://haveibeensquatted.com) - Domain security monitoring service

*Using clerk-rs in production? [Open a PR](https://github.com/DarrenBaldwin07/clerk-rs/pulls) to add your company to this list!*

## 💪 Contributing

Contributions are welcome and appreciated! Feel free to:

- Report bugs and issues
- Submit feature requests
- Improve documentation
- Submit pull requests

Before submitting a PR, please make sure to update tests as appropriate and ensure all tests pass.

## 📜 License

This project is licensed under the MIT License - see the LICENSE file for details.
