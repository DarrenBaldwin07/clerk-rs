[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# clerk-rs

> The official community-maintained Clerk SDK for Rust

`clerk-rs` is a comprehensive Rust SDK for the [Clerk](https://clerk.com) authentication platform. It provides a type-safe, ergonomic interface for integrating Clerk's powerful authentication and user management features into your Rust applications.

## Features

- 🔐 **Full API Coverage** - Complete implementation of the Clerk Backend API
- 🚀 **Web Framework Support** - Built-in middleware for Actix Web, Axum, Rocket, and Poem
- 🔑 **JWT Validation** - Secure JWT verification with JWKS caching
- 🍪 **Session Cookie Support** - Validate session cookies for same-origin requests
- ⚡ **Async/Await** - Built on tokio for high-performance async operations
- 🛡️ **Type-Safe** - Leverage Rust's type system for compile-time correctness

## Installation

Add `clerk-rs` to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4"
```

### Framework-Specific Features

Enable the feature flag for your web framework:

```toml
# For Actix Web
clerk-rs = { version = "0.4", features = ["actix"] }

# For Axum
clerk-rs = { version = "0.4", features = ["axum"] }

# For Rocket
clerk-rs = { version = "0.4", features = ["rocket"] }

# For Poem
clerk-rs = { version = "0.4", features = ["poem"] }
```

## Quick Start

First, obtain your Clerk secret key from the [Clerk Dashboard](https://dashboard.clerk.com). Then initialize the client:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

let config = ClerkConfiguration::new(
    None,
    None,
    Some("sk_test_your_secret_key".to_string()),
    None
);
let client = Clerk::new(config);
```

## Documentation

- [Official Clerk Backend API Documentation](https://clerk.com/docs/reference/backend-api)
- [clerk-rs SDK API Documentation](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)
- [API Reference on docs.rs](https://docs.rs/clerk-rs)

> **Note:** This SDK is actively maintained to stay in sync with the Clerk API. If you notice any discrepancies or issues, please [open an issue](https://github.com/DarrenBaldwin07/clerk-rs/issues).

## Usage Examples

Explore complete examples in the [`/examples`](./examples) directory.

### Basic API Usage

#### Making Direct HTTP Requests

Use the low-level HTTP interface for direct API access:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(
        None,
        None,
        Some("sk_test_key".to_string()),
        None
    );
    let client = Clerk::new(config);

    // Fetch list of users
    let user_list = client.get(ClerkGetEndpoint::GetUserList).await?;

    Ok(())
}
```

#### Using High-Level API Methods

Use the convenient API methods for common operations:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(
        None,
        None,
        Some("sk_test_key".to_string()),
        None
    );
    let client = Clerk::new(config);

    // Create an email using the high-level API
    Email::create(&client, Some(your_clerk_email)).await?;

    Ok(())
}
```

### Framework Integration

Protect your web application routes with Clerk authentication middleware.

#### Actix Web

Enable the `actix` feature and use the `ClerkMiddleware`:

```rust
use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> impl Responder {
    "This route is protected by Clerk authentication!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let config = ClerkConfiguration::new(
            None,
            None,
            Some("your_secret_key".to_string()),
            None
        );
        let clerk = Clerk::new(config);

        App::new()
            .wrap(ClerkMiddleware::new(
                MemoryCacheJwksProvider::new(clerk),
                None,
                true  // validate_session_cookie
            ))
            .route("/protected", web::get().to(protected_route))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

#### Axum

Enable the `axum` feature and use the `ClerkLayer`:

```rust
use axum::{routing::get, Router};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> &'static str {
    "This route is protected by Clerk authentication!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = ClerkConfiguration::new(
        None,
        None,
        Some("your_secret_key".to_string()),
        None
    );
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/protected", get(protected_route))
        .layer(ClerkLayer::new(
            MemoryCacheJwksProvider::new(clerk),
            None,
            true  // validate_session_cookie
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
```

#### Rocket

Enable the `rocket` feature and use the `ClerkGuard`:

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
    "This route is protected by Clerk authentication!"
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
        true,  // validate_session_cookie
    );

    rocket::build()
        .mount("/", routes![protected_route])
        .manage(clerk_config)
}
```

#### Poem

Enable the `poem` feature (requires Poem v3) and use the `ClerkPoemMiddleware`:

```rust
use clerk_rs::{
    clerk::Clerk,
    validators::{jwks::MemoryCacheJwksProvider, poem::ClerkPoemMiddleware},
    ClerkConfiguration,
};
use poem::{get, handler, listener::TcpListener, EndpointExt, Route, Server};

#[handler]
fn protected_route() -> &'static str {
    "This route is protected by Clerk authentication!"
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

    // Initialize the Clerk middleware
    let clerk_middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        true,  // validate_session_cookie
        // Optional: exclude specific routes from authentication
        Some(vec!["/public".to_string()]),
    );

    let app = Route::new()
        .at("/protected", get(protected_route))
        .with(clerk_middleware);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

**Note:** Access the validated JWT in your handlers using `Data<&ClerkJwt>` or `req.data::<ClerkJwt>()`.

## Roadmap

We're continuously improving `clerk-rs`. Here's what's on the horizon:

- [ ] Support for additional HTTP clients (e.g., hyper)
- [ ] Support for tokio and async-std runtimes with hyper
- [ ] Optional blocking reqwest client
- [ ] Additional web framework middleware (e.g., warp)
- [x] Session cookie validation for same-origin requests

## Companies Using clerk-rs in Production

- [Tembo](https://tembo.io) - Managed Postgres service
- [Rezon](https://rezon.ai) - AI-powered solutions
- [Gitar](https://gitar.co) - Developer productivity tools
- [Have I Been Squatted](https://haveibeensquatted.com) - Package security service

**Using clerk-rs in production?** We'd love to feature your company! Open a PR to add yourself to this list.

## Contributing

Contributions are welcome! Please feel free to submit issues, create pull requests, or suggest improvements.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the [LICENSE.MD](LICENSE.MD) file for details.

## Support

- [GitHub Issues](https://github.com/DarrenBaldwin07/clerk-rs/issues) - Report bugs or request features
- [Documentation](https://docs.rs/clerk-rs) - Read the full API documentation
- [Clerk Documentation](https://clerk.com/docs) - Learn more about Clerk

---

Made with ❤️ by the community
