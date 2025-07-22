<div align="center">
  <h1>clerk-rs</h1>
  <p><strong>The official community-maintained Rust SDK for Clerk</strong></p>
  <p>
    <a href="https://crates.io/crates/clerk-rs"><img src="https://img.shields.io/crates/v/clerk-rs?style=flat-square" alt="crates.io"></a>
    <a href="https://crates.io/crates/clerk-rs"><img src="https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square" alt="Downloads"></a>
    <a href="https://docs.rs/clerk-rs"><img src="https://img.shields.io/docsrs/clerk-rs?style=flat-square" alt="docs.rs"></a>
    <a href="https://github.com/DarrenBaldwin07/clerk-rs/blob/main/LICENSE.MD"><img src="https://img.shields.io/crates/l/clerk-rs?style=flat-square" alt="License"></a>
  </p>
</div>

## 🔐 Authentication for Rust, simplified

**clerk-rs** is a powerful, type-safe Rust SDK for [Clerk](https://clerk.com), the complete authentication and user management platform for modern applications. This community-maintained SDK provides seamless integration with Clerk's robust authentication system in your Rust backend services.

### Why clerk-rs?

- **Type-safe API**: Fully typed interfaces matching Clerk's API specifications
- **Framework integration**: Native middleware for Actix, Axum, Rocket, and Poem
- **Secure JWT validation**: Built-in JWKS caching and verification
- **Complete API coverage**: Support for all Clerk backend API endpoints
- **Modern Rust**: Async-first design using Rust's async/await syntax
- **Well-maintained**: Regularly updated to match Clerk's API changes

## 🚀 Quick Start

Add clerk-rs to your Cargo.toml:

```toml
[dependencies]
clerk-rs = "0.4.1"
```

For framework-specific middleware support, enable the corresponding feature:

```toml
# For Actix support
clerk-rs = { version = "0.4.1", features = ["actix"] }

# For Axum support
clerk-rs = { version = "0.4.1", features = ["axum"] }

# For Rocket support
clerk-rs = { version = "0.4.1", features = ["rocket"] }

# For Poem support
clerk-rs = { version = "0.4.1", features = ["poem"] }
```

## 🔧 Usage Examples

### Basic API Usage

Create a Clerk client and make authenticated requests:

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::users_api::User};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Clerk with your secret key
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);
    
    // Fetch a list of users
    let users = User::get_list(&client, None, None, None, None, None).await?;
    println!("Found {} users", users.len());
    
    Ok(())
}
```

### Actix Web Integration

Protect your Actix Web routes with Clerk authentication:

```rust
use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> impl Responder {
    "If you see this, you're authenticated!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize Clerk client
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);
    
    // Set up server with Clerk middleware
    HttpServer::new(|| {
        App::new()
            .wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
            .route("/protected", web::get().to(protected_route))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### Axum Integration

Add Clerk authentication to your Axum application:

```rust
use axum::{routing::get, Router};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> &'static str {
    "If you see this, you're authenticated!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Initialize Clerk client
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);
    
    // Create router with Clerk layer
    let app = Router::new()
        .route("/protected", get(protected_route))
        .layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), None, true));
    
    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
```

### Rocket Integration

Secure your Rocket routes with Clerk:

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
fn protected_route(jwt: ClerkGuard<MemoryCacheJwksProvider>) -> &'static str {
    "If you see this, you're authenticated!"
}

#[launch]
fn rocket() -> _ {
    // Initialize Clerk
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);
    
    // Configure Clerk guard
    let clerk_config = ClerkGuardConfig::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true, // validate_session_cookie
    );
    
    // Create Rocket instance
    rocket::build()
        .mount("/", routes![protected_route])
        .manage(clerk_config)
}
```

### Poem Integration

Add authentication to your Poem application:

```rust
use clerk_rs::{
    clerk::Clerk,
    validators::{jwks::MemoryCacheJwksProvider, poem::ClerkPoemMiddleware},
    ClerkConfiguration,
};
use poem::{get, handler, listener::TcpListener, EndpointExt, Route, Server};

#[handler]
fn protected_route() -> String {
    "If you see this, you're authenticated!".to_string()
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Initialize Clerk
    let clerk = Clerk::new(ClerkConfiguration::new(
        None,
        None,
        Some("your_secret_key".to_string()),
        None,
    ));
    
    // Create middleware
    let clerk_middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        true,
        None,
    );
    
    // Set up routes with middleware
    let app = Route::new()
        .at("/protected", get(protected_route))
        .with(clerk_middleware);
    
    // Start server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

## 📚 Documentation

For comprehensive documentation, please check:

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)
- [clerk-rs examples](/examples)

## 🏢 Production Users

clerk-rs is trusted by organizations worldwide:

- [Tembo](https://tembo.io) - PostgreSQL platform for developers
- [Rezon](https://rezon.ai) - AI-powered customer engagement
- [Gitar](https://gitar.co) - Version control for API design
- [Have I Been Squatted](https://haveibeensquatted.com) - Domain security monitoring

*Using clerk-rs in production? Open a PR to add your company here!*

## 🛣️ Roadmap

- [ ] Support additional HTTP clients alongside reqwest (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via \_\_session cookie on same-origin
- [ ] Improved documentation and examples

## 🤝 Contributing

We welcome contributions from the community! If you notice anything that needs updating or is not aligned with the official Clerk API, please open an issue or submit a pull request.

## 📄 License

clerk-rs is licensed under the MIT License - see the [LICENSE](LICENSE.MD) file for details.