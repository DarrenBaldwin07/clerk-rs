# clerk-rs

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.MD)

**The official community-maintained Clerk SDK for Rust**

A comprehensive Rust SDK for integrating with [Clerk](https://clerk.com), providing authentication and user management capabilities for your applications.

## 🚀 Quick Start

Add `clerk-rs` to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4"
```

## ✨ Features

- **Complete API Coverage**: Full access to Clerk's Backend API
- **Framework Integration**: Built-in middleware for popular web frameworks
- **JWT Validation**: Secure token verification with JWKS support
- **Session Management**: Cookie-based session validation
- **Type Safety**: Comprehensive Rust type definitions
- **Async Support**: Built on `tokio` with `reqwest`

### Supported Frameworks

| Framework | Feature Flag | Status |
|-----------|--------------|--------|
| Actix Web | `actix` | ✅ |
| Axum | `axum` | ✅ |
| Rocket | `rocket` | ✅ |
| Poem | `poem` | ✅ |

## 📖 Documentation

- [API Documentation](https://docs.rs/clerk-rs)
- [Clerk Backend API Reference](https://clerk.com/docs/reference/backend-api)
- [SDK API Documentation](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

## 🎯 Usage Examples

### Basic Client Setup

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(
        None, // api_url
        None, // api_version  
        Some("sk_test_your_key_here".to_string()), // secret_key
        None  // jwt_key
    );
    let client = Clerk::new(config);
    
    // Use the client...
    Ok(())
}
```

### HTTP Endpoint Integration

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    let users = client.get(ClerkGetEndpoint::GetUserList).await?;
    println!("Retrieved users: {:?}", users);

    Ok(())
}
```

### Using SDK Methods

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Send an email
    Email::create(&client, Some(email_request)).await?;

    Ok(())
}
```

## 🛡️ Authentication Middleware

### Actix Web

Enable the `actix` feature and protect your routes:

```rust
use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> impl Responder {
    "This route is protected!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    HttpServer::new(move || {
        App::new()
            .wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk.clone()), None, true))
            .route("/protected", web::get().to(protected_route))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```

### Axum

Enable the `axum` feature:

```rust
use axum::{response::Html, routing::get, Router};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> Html<&'static str> {
    Html("<h1>Protected Route</h1>")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/protected", get(protected_route))
        .layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), None, true));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```

### Rocket

Enable the `rocket` feature:

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

#[get("/")]
fn protected_route(_jwt: ClerkGuard<MemoryCacheJwksProvider>) -> &'static str {
    "Protected route accessed successfully!"
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

    rocket::build()
        .mount("/", routes![protected_route])
        .manage(clerk_config)
}
```

### Poem

Enable the `poem` feature:

```rust
use clerk_rs::{
    clerk::Clerk,
    validators::{jwks::MemoryCacheJwksProvider, poem::ClerkPoemMiddleware},
    ClerkConfiguration,
};
use poem::{get, handler, listener::TcpListener, web::Path, EndpointExt, Route, Server};

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("Hello, {}! This route is protected.", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);
    
    let middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        true,
        Some(vec!["/health".to_owned()]), // Exclude health check from auth
    );

    let app = Route::new()
        .at("/hello/:name", get(hello))
        .with(middleware);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

> **Note**: Access JWT data using `Data<&ClerkJwt>` or `req.data::<ClerkJwt>()` in your handlers.

## 📁 Examples

Comprehensive examples are available in the [`/examples`](./examples) directory:

- [`http.rs`](./examples/http.rs) - Basic HTTP client usage
- [`api.rs`](./examples/api.rs) - SDK method examples  
- [`actix.rs`](./examples/actix.rs) - Actix Web integration
- [`axum.rs`](./examples/axum.rs) - Axum integration
- [`rocket.rs`](./examples/rocket.rs) - Rocket integration

Run examples with:
```bash
cargo run --example http --features rustls-tls
cargo run --example actix --features actix,rustls-tls
```

## 🗺️ Roadmap

- [ ] **HTTP Client Flexibility**: Support for hyper and other HTTP clients
- [ ] **Runtime Support**: Tokio and async-std compatibility for hyper
- [ ] **Blocking Client**: Optional synchronous reqwest client
- [x] **Session Cookies**: Authorization via `__session` cookie (same-origin)
- [ ] **Framework Expansion**: Additional web framework support

## 🏭 Production Users

Companies using `clerk-rs` in production:

- [**Tembo**](https://tembo.io) - Postgres platform
- [**Rezon**](https://rezon.ai) - AI platform  
- [**Gitar**](https://gitar.co) - Development tools
- [**Have I Been Squatted**](https://haveibeensquatted.com) - Security service

> Using clerk-rs in production? [Open a PR](https://github.com/DarrenBaldwin07/clerk-rs/pulls) to add your company!

## 🤝 Contributing

We welcome contributions! This SDK is updated frequently to stay in sync with Clerk's API. If you notice any discrepancies or have suggestions:

1. [Open an issue](https://github.com/DarrenBaldwin07/clerk-rs/issues/new)
2. Submit a pull request
3. Check our [contribution guidelines](./CONTRIBUTING.md)

## 📄 License

This project is licensed under the [MIT License](./LICENSE.MD).
