# clerk-rs

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

The official community-maintained Clerk SDK for Rust. This SDK provides comprehensive support for Clerk's authentication and user management services, with built-in middleware for popular Rust web frameworks.

## Features

- ✅ Complete Clerk API coverage
- ✅ Framework integrations (Actix, Axum, Rocket, Poem)
- ✅ JWT validation and session management
- ✅ Type-safe API client
- ✅ Async/await support
- ✅ Production-ready with caching

## Quick Start

Add clerk-rs to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.1"
```

## Documentation

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [Clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

> **Note:** This SDK is actively maintained and updated to stay in sync with the official Clerk API. If you notice any discrepancies or need updates, please [open an issue](https://github.com/DarrenBaldwin07/clerk-rs/issues)!

## Usage Examples

### Basic API Client

Create a Clerk client and make API requests:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Clerk client with your secret key
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("sk_test_your_secret_key".to_string()), 
        None
    );
    let client = Clerk::new(config);

    // Fetch user list
    let users = client.get(ClerkGetEndpoint::GetUserList).await?;
    println!("Retrieved {} users", users.len());

    Ok(())
}
```

### High-level API Methods

Use convenient high-level methods for common operations:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("sk_test_your_secret_key".to_string()), 
        None
    );
    let client = Clerk::new(config);

    // Send an email
    Email::create(&client, Some(your_email_request)).await?;

    Ok(())
}
```

## Framework Integrations

### Actix Web

Enable the `actix` feature in your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = { version = "0.1", features = ["actix"] }
```

```rust
use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_handler() -> impl Responder {
    "Hello, authenticated user!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("sk_test_your_secret_key".to_string()), 
        None
    );
    let clerk = Clerk::new(config);

    HttpServer::new(move || {
        App::new()
            .wrap(ClerkMiddleware::new(
                MemoryCacheJwksProvider::new(clerk.clone()), 
                None, 
                true
            ))
            .route("/protected", web::get().to(protected_handler))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### Axum

```toml
[dependencies]
clerk-rs = { version = "0.1", features = ["axum"] }
```

```rust
use axum::{routing::get, Router};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_handler() -> &'static str {
    "Hello, authenticated user!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("sk_test_your_secret_key".to_string()), 
        None
    );
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/protected", get(protected_handler))
        .layer(ClerkLayer::new(
            MemoryCacheJwksProvider::new(clerk), 
            None, 
            true
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
```

### Rocket

```toml
[dependencies]
clerk-rs = { version = "0.1", features = ["rocket"] }
```

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
fn protected_handler(_jwt: ClerkGuard<MemoryCacheJwksProvider>) -> &'static str {
    "Hello, authenticated user!"
}

#[launch]
fn rocket() -> _ {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("sk_test_your_secret_key".to_string()), 
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

### Poem

```toml
[dependencies]
clerk-rs = { version = "0.1", features = ["poem"] }
poem = "3.0"
```
```rust
use clerk_rs::{
    clerk::Clerk,
    validators::{jwks::MemoryCacheJwksProvider, poem::ClerkPoemMiddleware},
    ClerkConfiguration,
};
use poem::{get, handler, listener::TcpListener, web::Path, EndpointExt, Route, Server};

#[handler]
fn protected_handler(Path(name): Path<String>) -> String {
    format!("Hello, authenticated user: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = ClerkConfiguration::new(
        None,
        None,
        Some("sk_test_your_secret_key".to_string()),
        None,
    );
    let clerk = Clerk::new(config);

    let clerk_middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        true,
        // Optional: exclude routes from auth verification
        Some(vec!["/health".to_string()]),
    );

    let app = Route::new()
        .at("/protected/:name", get(protected_handler))
        .with(clerk_middleware);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

> **Note:** The JWT can be accessed using `Data<&ClerkJwt>` or `req.data::<ClerkJwt>()`.

## More Examples

Check out complete examples in the [`/examples`](./examples) directory for detailed implementation guides.

## Roadmap

- [ ] Support additional HTTP clients (hyper, etc.)
- [ ] Multiple async runtime support (tokio, async-std)
- [ ] Optional blocking client support
- [x] Session cookie validation
- [ ] Additional framework integrations (warp, tide)

## Production Users

- [Tembo](https://tembo.io) - Cloud Postgres platform
- [Rezon](https://rezon.ai) - AI-powered solutions
- [Gitar](https://gitar.co) - Developer productivity tools
- [Have I Been Squatted](https://haveibeensquatted.com) - Domain security checker

[Add your company here →](https://github.com/DarrenBaldwin07/clerk-rs/pulls)

## Contributing

We welcome contributions! Please see our [contributing guidelines](./CONTRIBUTING.md) for details.

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.
