[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# clerk-rs: The Official Community-Maintained Clerk SDK for Rust

`clerk-rs` is a comprehensive Rust SDK for interacting with [Clerk](https://clerk.com), providing secure authentication and user management for your Rust applications. This SDK supports multiple web frameworks including Actix Web, Axum, Rocket, and Poem.

## Features

- **Complete API Coverage**: Full support for Clerk's Backend API
- **Framework Integration**: First-class support for popular Rust web frameworks
- **Type Safety**: Fully typed API for a better developer experience
- **Async/Await**: Built with modern Rust's async ecosystem in mind
- **Middleware Support**: Easy-to-use authentication middleware for protecting routes

## Installation

Add `clerk-rs` to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4.1"

# Optional framework-specific features
clerk-rs = { version = "0.4.1", features = ["actix"] }  # For Actix Web
clerk-rs = { version = "0.4.1", features = ["axum"] }   # For Axum
clerk-rs = { version = "0.4.1", features = ["rocket"] } # For Rocket
clerk-rs = { version = "0.4.1", features = ["poem"] }   # For Poem
```

## Quick Start

### Basic API Request

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Clerk with your secret key
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Make an API request
    let res = client.get(ClerkGetEndpoint::GetUserList).await?;

    Ok(())
}
```

### Using Specific API Methods

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Use a specific API method
    Email::create(&client, Some(your_clerk_email));

    Ok(())
}
```

## Framework Integration

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

### Rocket

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

### Poem

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
    
    // Initialize middleware
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

## Documentation

For comprehensive documentation:

- [Official Clerk Backend API Documentation](https://clerk.com/docs/reference/backend-api)
- [clerk-rs SDK API Documentation](https://docs.rs/clerk-rs)
- [SDK API Reference](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

## Examples

Check the [`examples/`](https://github.com/DarrenBaldwin07/clerk-rs/tree/main/examples) directory for complete working examples.

## Roadmap

- [x] Framework support for Actix Web, Axum, Rocket, and Poem
- [x] Support authorization via \_\_session cookie on same-origin
- [ ] Support other HTTP clients along with the default reqwest client (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [ ] Add validator support for additional frameworks

## Contributing

Contributions are welcome! This SDK is regularly updated to keep up with any changes to the official Clerk API. If you find something that needs updating or is not aligned with the official Clerk API, please open an issue or submit a pull request.

## Production Users

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)
- Open a PR and add your company here :)

## License

This project is licensed under the MIT License - see the LICENSE file for details.