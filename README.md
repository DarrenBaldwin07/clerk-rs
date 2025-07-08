[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# clerk-rs: The Official Community-Maintained Clerk SDK for Rust

![Clerk + Rust](https://img.shields.io/badge/Clerk%20%2B%20Rust-Auth%20Made%20Easy-8A2BE2?style=flat-square)

A robust Rust SDK for integrating [Clerk](https://clerk.com/) authentication and user management into your Rust applications. This community-maintained SDK provides idiomatic Rust interfaces to Clerk's Backend API and supports multiple web frameworks including Actix Web, Axum, Rocket, and Poem.

With clerk-rs, you can quickly add secure authentication, user management, and session handling to your Rust web applications with minimal boilerplate code.

## Features

- Complete API coverage of Clerk's Backend API
- Framework-specific integrations for:
  - Actix Web
  - Axum
  - Rocket
  - Poem
- JWT validation and session management
- Memory-cached JWKS provider for efficient authentication
- Type-safe API client
- Comprehensive error handling
- Session cookie support
- Configurable authentication options

## Documentation

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)
- [Examples directory](/examples)
- [API Reference on docs.rs](https://docs.rs/clerk-rs)

> This SDK is actively maintained to stay in sync with the official Clerk API. If you find any discrepancies or missing features, please [open an issue](https://github.com/DarrenBaldwin07/clerk-rs/issues)!

## Quick Start

Get started with clerk-rs in three simple steps:

1. **Sign up for Clerk** at [clerk.com](https://clerk.com/) and create an application to obtain your API keys.
2. **Add clerk-rs** to your Rust project (see Installation section below).
3. **Integrate authentication** using the examples for your preferred web framework.

## Installation

Add clerk-rs to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.1"  # Check crates.io for the latest version
```

To enable framework-specific features:

```toml
[dependencies]
clerk-rs = { version = "0.1", features = ["actix", "axum", "rocket", "poem"] }
```

## Usage Examples

### Basic HTTP Request

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    let users = client.get(ClerkGetEndpoint::GetUserList).await?;
    println!("Retrieved user list successfully");
    
    Ok(())
}
```

### Using High-Level API Methods

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);
    
    // Example email data structure (replace with actual fields from the API)
    let email_data = serde_json::json!({
        "from_email_name": "support",
        "subject": "Welcome to our service",
        "body": "Thanks for signing up!",
        "email_address_id": "email_address_id_value"
    });
    
    Email::create(&client, Some(email_data)).await?;

    Ok(())
}
```

### Protecting Routes in Actix Web

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

### Protecting Routes in Axum

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

### Protecting Routes in Rocket

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
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);
    let clerk_config = ClerkGuardConfig::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true, // validate_session_cookie
    );

    rocket::build().mount("/", routes![index]).manage(clerk_config)
}
```

### Protecting Routes in Poem

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
        Some("your_secret_key".to_owned()),
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

The JWT can be accessed using `Data<&ClerkJwt>` (or `req.data::<ClerkJwt>()`). This gives you access to the decoded JWT payload containing user information and claims.

## Roadmap

- [ ] Support other HTTP clients along with the default reqwest client (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via \_\_session cookie on same-origin
- [x] Add validator support for axum, rocket, poem
- [ ] Improved error handling and custom error types
- [ ] Enhanced documentation with more examples
- [ ] Frontend integration examples for popular Rust WASM frameworks

## Production Users

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)

Want to be listed here? Open a PR and add your company! We'd love to showcase how clerk-rs is being used in production.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. Check out the roadmap for areas that need attention or propose your own improvements.

## Support

If you encounter any issues or have questions, please [open an issue](https://github.com/DarrenBaldwin07/clerk-rs/issues) on GitHub.

## License

This project is licensed under the [MIT License](LICENSE).