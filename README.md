[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# Clerk-rs: The Official Community-Maintained Clerk SDK for Rust

Clerk-rs provides a comprehensive Rust SDK for the [Clerk](https://clerk.com) authentication and user management API. This SDK makes it easy to integrate Clerk's authentication services into your Rust applications, with built-in support for popular frameworks like Actix, Axum, Rocket, and Poem.

## Documentation

- [Official Clerk Backend API Documentation](https://clerk.com/docs/reference/backend-api)
- [Clerk-rs SDK API Documentation](https://docs.rs/clerk-rs)

> This SDK is actively maintained to stay in sync with the official Clerk API. If you notice any discrepancies or missing features, please [open an issue](https://github.com/DarrenBaldwin07/clerk-rs/issues).

## Features

- Complete Clerk API coverage
- JWT validation and session management
- Framework-specific middleware/guards for:
  - Actix Web
  - Axum
  - Rocket
  - Poem
- Flexible configuration options
- Type-safe API interfaces

## Installation

Add clerk-rs to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4.1"
```

With framework-specific features:

```toml
[dependencies]
clerk-rs = { version = "0.4.1", features = ["actix"] }  # For Actix Web
clerk-rs = { version = "0.4.1", features = ["axum"] }   # For Axum
clerk-rs = { version = "0.4.1", features = ["rocket"] } # For Rocket
clerk-rs = { version = "0.4.1", features = ["poem"] }   # For Poem
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

    let res = client.get(ClerkGetEndpoint::GetUserList).await?;

    Ok(())
}
```

### Using Clerk API Methods

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

### Actix Web Integration

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

### Axum Integration

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

### Rocket Integration

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

### Poem Integration

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

The JWT can be accessed using `Data<&ClerkJwt>` (or `req.data::<ClerkJwt>()`).

## Additional Examples

Check out more usage examples in the [`/examples`](https://github.com/DarrenBaldwin07/clerk-rs/tree/main/examples) directory.

## Roadmap

- [ ] Support other HTTP clients along with the default reqwest client (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via \_\_session cookie on same-origin
- [x] Add validator support for axum, rocket
- [ ] Add validator support for warp

## Production Users

The following companies and projects use clerk-rs in production:

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)

Using clerk-rs in production? Open a PR to add your company here!

## License

This project is licensed under the MIT License.