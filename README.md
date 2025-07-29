[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# The official community-maintained Clerk SDK for Rust

## Overview

**clerk-rs** is the official community-maintained Rust SDK for [Clerk](https://clerk.com), a comprehensive authentication and user management platform. This library provides Rust developers with a robust, type-safe interface to integrate Clerk's authentication services into their applications.

## What is clerk-rs?

clerk-rs is a comprehensive Rust client library that enables developers to:

- **Authenticate users** through Clerk's authentication system
- **Manage user accounts** programmatically via the Clerk Backend API
- **Protect API endpoints** with JWT validation and session management
- **Integrate seamlessly** with popular Rust web frameworks like Actix-web, Axum, Rocket, and Poem

## Key Features

### 🔐 Complete Authentication Solution
- JWT token validation and verification
- Session cookie validation for same-origin requests
- JWKS (JSON Web Key Set) caching for performance
- Support for multiple authentication methods

### 🌐 Framework Integration
The SDK provides middleware and guards for popular Rust web frameworks:
- **Actix-web**: `ClerkMiddleware` for route protection
- **Axum**: `ClerkLayer` for application-wide authentication
- **Rocket**: `ClerkGuard` for request guards
- **Poem**: `ClerkPoemMiddleware` for endpoint protection

### 📡 Full API Coverage
Complete bindings for the Clerk Backend API including:
- User management (create, update, delete users)
- Organization management
- Session handling
- Email and SMS operations
- JWT template management
- Webhook validation

### 🔧 Developer Experience
- **Type-safe**: All API responses and requests are strongly typed
- **Async/await**: Built on modern async Rust patterns
- **Comprehensive examples**: Ready-to-use examples for each supported framework
- **Well-documented**: Extensive documentation and API reference

## Use Cases

clerk-rs is perfect for:

- **Web APIs** that need user authentication and authorization
- **Microservices** requiring centralized user management
- **SaaS applications** with multi-tenant architecture
- **Backend services** that integrate with Clerk-powered frontend applications
- **Enterprise applications** requiring robust authentication flows

## Architecture

The library is structured into several key modules:

- **`apis/`**: Generated API client bindings for all Clerk endpoints
- **`models/`**: Strongly-typed data structures for API requests/responses
- **`validators/`**: Framework-specific middleware for authentication
- **`clerk.rs`**: Core client implementation with HTTP methods
- **`endpoints.rs`**: Enum definitions for all available API endpoints

---

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
