[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# The official community-maintained Clerk SDK for Rust

## Overview

**Clerk-rs** is the official community-maintained Rust SDK for [Clerk](https://clerk.com), a complete authentication and user management solution. This SDK provides comprehensive Rust bindings for the Clerk Backend API, enabling developers to integrate Clerk's authentication services seamlessly into their Rust applications.

## What is Clerk-rs?

Clerk-rs is a full-featured authentication SDK that bridges the gap between Rust applications and Clerk's powerful authentication platform. It offers both low-level API access and high-level middleware integrations for popular Rust web frameworks.

### Key Features

- **Complete API Coverage**: Full Rust bindings for the entire Clerk Backend API
- **Framework Integration**: Built-in middleware support for major Rust web frameworks:
  - **Actix Web**: ClerkMiddleware for request authentication
  - **Axum**: ClerkLayer for tower-based authentication 
  - **Rocket**: ClerkGuard for request guards
  - **Poem**: ClerkPoemMiddleware for poem framework
- **JWT Validation**: Comprehensive JWT token validation and verification
- **Session Management**: Handle user sessions and authentication state
- **Organization Support**: Full support for Clerk's organization features
- **User Management**: Complete user CRUD operations and profile management
- **Email & SMS**: Send transactional emails and SMS messages
- **Webhook Support**: Handle Clerk webhook events
- **Memory-efficient**: Built-in JWKS caching with MemoryCacheJwksProvider

### Core Functionality

#### 1. **API Client**
The core `Clerk` struct provides methods for all HTTP operations:
- GET, POST, PUT, DELETE, PATCH requests
- Parameter-based dynamic endpoints
- Comprehensive error handling
- Async/await support throughout

#### 2. **Authentication Middleware**
Framework-specific middleware that automatically:
- Validates JWT tokens from Authorization headers
- Verifies session cookies for same-origin requests
- Provides authenticated user context to route handlers
- Handles JWKS (JSON Web Key Set) fetching and caching

#### 3. **User & Organization Management**
Complete APIs for managing:
- User creation, updates, and deletion
- Email addresses and phone numbers
- User metadata and profiles
- Organizations and memberships
- Invitations and role management

#### 4. **Communication Services**
- Email API for transactional emails
- SMS API for phone verification
- Template management for customized messages

## Architecture

Clerk-rs is built with a modular architecture:

```
src/
├── apis/           # API endpoint implementations
├── models/         # Data models and structures  
├── validators/     # Framework-specific middleware
├── clerk.rs        # Core client implementation
├── endpoints.rs    # Endpoint definitions
└── lib.rs         # Public API exports
```

### Framework Support

Each supported framework gets its own validator module with idiomatic integration:

- **Actix**: Middleware that wraps routes with authentication
- **Axum**: Tower layer for the axum ecosystem  
- **Rocket**: Request guard for route protection
- **Poem**: Endpoint middleware for poem applications

## Production Ready

Clerk-rs is actively used in production by several companies including:
- [Tembo](https://tembo.io) - Postgres cloud platform
- [Rezon](https://rezon.ai) - AI platform
- [Gitar](https://gitar.co) - Git analytics
- [Have I Been Squatted](https://haveibeensquatted.com) - Domain security

## Why Clerk-rs?

### For Rust Developers
- **Type Safety**: Leverages Rust's type system for compile-time API correctness
- **Performance**: Efficient async HTTP client with connection pooling
- **Memory Safety**: Zero-cost abstractions with Rust's ownership model
- **Ecosystem Integration**: Works seamlessly with the Rust web ecosystem

### For Authentication
- **Comprehensive**: Covers all aspects of modern authentication
- **Secure**: Built-in JWT validation and session management
- **Scalable**: Designed for high-traffic production applications
- **Developer Experience**: Clean APIs and extensive documentation

## Getting Started

Add clerk-rs to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = { version = "0.4", features = ["axum"] }
```

Basic usage:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

let config = ClerkConfiguration::new(
    None, None, 
    Some("sk_test_your_secret_key".to_string()), 
    None
);
let client = Clerk::new(config);

// Get user list
let users = client.get(ClerkGetEndpoint::GetUserList).await?;
```

## Development Status

Clerk-rs is actively maintained and frequently updated to stay in sync with the official Clerk API. The project welcomes contributions and maintains compatibility with the latest Rust stable releases.

The SDK provides a robust foundation for any Rust application requiring modern authentication and user management capabilities.

For more detailed documentation, please reference the below links:

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [Clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

> This SDK is updated frequently to keep up with any changes to the actual Clerk API. If you see anything that needs updating or is not inline with the official Clerk api, please open an issue!

## Examples

> Check out examples in the `/examples` directory

### Using a traditional http request to a valid clerk endpoint:

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    let res = client.get(ClerkGetEndpoint::GetUserList).await?;

    Ok(())
}
```

### Using a clerk-rs method:

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    Email::create(&client, Some(your_clerk_email));

    Ok(())
}
```

### Protecting a actix-web endpoint with Clerk.dev:

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

### Protecting a axum endpoint with Clerk.dev:

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
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/index", get(index))
        .layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), None, true));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
```

### Protecting a rocket endpoint with Clerk.dev:

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
	let config = ClerkConfiguration::new(None, None, Some("sk_test_F9HM5l3WMTDMdBB0ygcMMAiL37QA6BvXYV1v18Noit".to_string()), None);
	let clerk = Clerk::new(config);
	let clerk_config = ClerkGuardConfig::new(
		MemoryCacheJwksProvider::new(clerk),
		None,
		true, // validate_session_cookie
	);

	rocket::build().mount("/", routes![index]).manage(clerk_config)
}

```

### Protecting a Poem endpoint with Clerk

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
    let clerk = Clerk::new(ClerkConfiguration::new(
        None,
        None,
        Some("sk_test_F9HM5l3WMTDMdBB0ygcMMAiL37QA6BvXYV1v18Noit".to_owned()),
        None,
    ));
    // Initialize middleware.
    let clerk_poem_middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk.clone()),
        true,
        // If you're using poem-openapi, you may need this to exclude some routes from auth
        // verification.
        Some(vec!["/some/route/to/exclude".to_owned()]),
    );

    let app = Route::new()
        .at("/hello/:name", get(hello))
        .with(clerk_poem_middleware); // Add middleware here (EndpointExt needs to be in scope).

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

The JWT can be accessed using `Data<&ClerkJwt>` (or `req.data::<ClerkJwt>()`).

## Roadmap

- [ ] Support other http clients along with the default reqwest client (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via \_\_session cookie on same-origin
- [ ] Add validator support for axum, rocket, warp

# Production users

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)
- Open a PR and add your company here :)

</br>
