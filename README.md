# clerk-rs

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

The official community-maintained Clerk SDK for Rust

## Features

- Complete Clerk Backend API support
- Built-in middleware for popular web frameworks:
  - Actix Web
  - Axum  
  - Rocket
  - Poem
- JWT token validation with JWKS support
- Session cookie validation
- Async/await support with Tokio
- Type-safe API with comprehensive error handling

## Quick Start

Add clerk-rs to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4"

# Optional: Enable framework-specific features
clerk-rs = { version = "0.4", features = ["axum"] }
```

## Documentation

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [Clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

> This SDK is kept up-to-date with the official Clerk API. Found something out of sync? [Open an issue](https://github.com/DarrenBaldwin07/clerk-rs/issues)!

## Usage Examples

### Basic Setup

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("sk_test_your_secret_key".to_string()), 
        None
    );
    let client = Clerk::new(config);
    
    // Use the client to make API calls
    Ok(())
}
```

### Making API Requests

#### Using HTTP Endpoints

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    let users = client.get(ClerkGetEndpoint::GetUserList).await?;
    println!("Users: {:?}", users);

    Ok(())
}
```

#### Using Typed API Methods

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    Email::create(&client, Some(your_clerk_email)).await?;

    Ok(())
}
```

## Web Framework Integration

### Actix Web

```toml
[dependencies]
clerk-rs = { version = "0.4", features = ["actix"] }
```

```rust
use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> impl Responder {
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
            .route("/protected", web::get().to(protected_route))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### Axum

```toml
[dependencies]
clerk-rs = { version = "0.4", features = ["axum"] }
```

```rust
use axum::{routing::get, Router};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> &'static str {
    "Hello, authenticated user!"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("sk_test_your_secret_key".to_string()), 
        None
    );
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/protected", get(protected_route))
        .layer(ClerkLayer::new(
            MemoryCacheJwksProvider::new(clerk), 
            None, 
            true
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```

### Rocket

```toml
[dependencies]
clerk-rs = { version = "0.4", features = ["rocket"] }
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

#[get("/protected")]
fn protected_route(_jwt: ClerkGuard<MemoryCacheJwksProvider>) -> &'static str {
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
        .mount("/", routes![protected_route])
        .manage(clerk_config)
}
```

### Poem

```toml
[dependencies]
clerk-rs = { version = "0.4", features = ["poem"] }
```

```rust
use clerk_rs::{
    clerk::Clerk,
    validators::{jwks::MemoryCacheJwksProvider, poem::ClerkPoemMiddleware},
    ClerkConfiguration,
};
use poem::{get, handler, listener::TcpListener, web::Path, EndpointExt, Route, Server};

#[handler]
fn protected_route(Path(name): Path<String>) -> String {
    format!("Hello, {}! You are authenticated.", name)
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
        Some(vec!["/health".to_string()]), // Exclude health check from auth
    );

    let app = Route::new()
        .at("/protected/:name", get(protected_route))
        .with(clerk_middleware);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

> **Note**: The JWT can be accessed using `Data<&ClerkJwt>` or `req.data::<ClerkJwt>()`.

## More Examples

Check out the complete examples in the [`/examples`](./examples) directory for more detailed implementations.

## Roadmap

- [ ] Support for additional HTTP clients (Hyper)
- [ ] Multiple async runtime support (Tokio, async-std)
- [ ] Optional blocking client support
- [x] Session cookie validation
- [ ] Additional web framework integrations

## Production Users

These companies are using clerk-rs in production:

- **[Tembo](https://tembo.io)** - Postgres cloud platform
- **[Rezon](https://rezon.ai)** - AI-powered development tools
- **[Gitar](https://gitar.co)** - Code analysis platform
- **[Have I Been Squatted](https://haveibeensquatted.com)** - Domain security service

Using clerk-rs in production? [Open a PR](https://github.com/DarrenBaldwin07/clerk-rs/pulls) to add your company!

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the [LICENSE.MD](LICENSE.MD) file for details.
