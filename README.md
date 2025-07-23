# clerk-rs

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)
[![MIT License](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](LICENSE.MD)

**The official community-maintained Clerk SDK for Rust**

A comprehensive Rust SDK for integrating with [Clerk](https://clerk.com), providing authentication, user management, and session handling for Rust applications. Built with support for popular web frameworks including Actix Web, Axum, Rocket, and Poem.

## ✨ Features

- 🔐 **Complete Authentication** - Full Clerk API integration with JWT validation
- 🌐 **Multi-Framework Support** - Built-in middleware for Actix Web, Axum, Rocket, and Poem  
- 🚀 **Async/Await Native** - Built for modern async Rust applications
- 📱 **Session Management** - Cookie and token-based session validation
- 🔒 **Security First** - JWKS validation with memory caching
- 📦 **Type Safe** - Full type definitions for all Clerk API endpoints
- 🛠️ **Flexible Configuration** - Multiple authentication methods and customizable settings

## 📖 Documentation

- [API Documentation](https://docs.rs/clerk-rs) - Complete Rust API docs
- [Clerk Backend API](https://clerk.com/docs/reference/backend-api) - Official Clerk API reference
- [SDK API Reference](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md) - Detailed endpoint documentation

## 🚀 Quick Start

Add `clerk-rs` to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4.1"
```

### Basic Configuration

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

let config = ClerkConfiguration::new(
    None,                                    // publishable_key
    None,                                    // domain  
    Some("sk_test_your_secret_key".to_string()), // secret_key
    None                                     // api_version
);
let client = Clerk::new(config);
```

## 📚 Usage Examples

### HTTP Client Usage

Make direct API calls to Clerk endpoints:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Fetch user list
    let users = client.get(ClerkGetEndpoint::GetUserList).await?;
    println!("Users: {:?}", users);

    Ok(())
}
```

### API Methods

Use high-level API methods for common operations:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Create an email
    Email::create(&client, Some(your_clerk_email)).await?;

    Ok(())
}
```

## 🛡️ Web Framework Integration

### Actix Web Middleware

Protect your Actix Web routes with Clerk authentication:

```toml
[dependencies]
clerk-rs = { version = "0.4.1", features = ["actix"] }
```

```rust
use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> impl Responder {
    "Hello authenticated user!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    HttpServer::new(move || {
        App::new()
            .wrap(ClerkMiddleware::new(
                MemoryCacheJwksProvider::new(clerk.clone()), 
                None, 
                true // validate_session_cookie
            ))
            .route("/protected", web::get().to(protected_route))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### Axum Integration

Add Clerk authentication to your Axum applications:

```toml
[dependencies] 
clerk-rs = { version = "0.4.1", features = ["axum"] }
```

```rust
use axum::{routing::get, Router};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_handler() -> &'static str {
    "Hello authenticated user!"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/protected", get(protected_handler))
        .layer(ClerkLayer::new(
            MemoryCacheJwksProvider::new(clerk), 
            None, 
            true // validate_session_cookie
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await?;
    Ok(())
}
```

### Rocket Integration

Use Clerk authentication guards in Rocket:

```toml
[dependencies]
clerk-rs = { version = "0.4.1", features = ["rocket"] }
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
fn protected_route(jwt: ClerkGuard<MemoryCacheJwksProvider>) -> &'static str {
    "Hello authenticated user!"
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

### Poem Integration

Integrate with Poem web framework using the built-in middleware:

```toml
[dependencies]
clerk-rs = { version = "0.4.1", features = ["poem"] }
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
    format!("Hello authenticated user: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);
    
    let clerk_middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        true, // validate_session_cookie
        Some(vec!["/health".to_owned()]), // excluded routes
    );

    let app = Route::new()
        .at("/hello/:name", get(protected_handler))
        .with(clerk_middleware);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

> **Note:** JWT data can be accessed using `Data<&ClerkJwt>` or `req.data::<ClerkJwt>()`.

## ⚙️ Configuration Options

### Feature Flags

Enable framework-specific features in your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = { version = "0.4.1", features = ["actix", "axum", "rocket", "poem"] }
```

Available features:
- `actix` - Actix Web middleware support
- `axum` - Axum layer support  
- `rocket` - Rocket guard support
- `poem` - Poem middleware support
- `rustls-tls` - Use rustls for TLS (default)
- `native-tls` - Use native TLS implementation

### Environment Variables

You can configure Clerk using environment variables:

```bash
CLERK_SECRET_KEY=sk_test_your_secret_key
CLERK_PUBLISHABLE_KEY=pk_test_your_publishable_key
```

## 🏗️ Project Structure

- `/src/apis/` - Generated API client code for all Clerk endpoints
- `/src/models/` - Type definitions for Clerk API models
- `/src/validators/` - Framework-specific authentication middleware
- `/examples/` - Usage examples for different frameworks
- `/docs/` - Detailed API documentation

## 🗺️ Roadmap

- [ ] Support for additional HTTP clients (hyper, surf)
- [ ] Async runtime flexibility (tokio, async-std)  
- [ ] Optional blocking client support
- [x] Session cookie validation support
- [ ] WebAssembly (WASM) compatibility
- [ ] Additional framework integrations (warp, tide)

## 🚀 Production Users

Companies using `clerk-rs` in production:

- [**Tembo**](https://tembo.io) - Postgres development platform
- [**Rezon**](https://rezon.ai) - AI-powered analytics
- [**Gitar**](https://gitar.co) - Git workflow automation
- [**Have I Been Squatted**](https://haveibeensquatted.com) - Domain security monitoring

*Using clerk-rs in production? [Open a PR](https://github.com/DarrenBaldwin07/clerk-rs/pulls) to add your company!*

## 🤝 Contributing

We welcome contributions! Please see our [contribution guidelines](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/CONTRIBUTING.md) for details.

### Development Setup

```bash
# Clone the repository
git clone https://github.com/DarrenBaldwin07/clerk-rs.git
cd clerk-rs

# Install dependencies
cargo build

# Run tests
cargo test

# Run examples
cargo run --example http --features=["actix"]
```

## 📄 License

This project is licensed under the MIT License - see the [LICENSE.MD](LICENSE.MD) file for details.

## 🔗 Links

- [Crates.io](https://crates.io/crates/clerk-rs)
- [Documentation](https://docs.rs/clerk-rs)
- [GitHub Repository](https://github.com/DarrenBaldwin07/clerk-rs)
- [Clerk Documentation](https://clerk.com/docs)
- [Issue Tracker](https://github.com/DarrenBaldwin07/clerk-rs/issues)

---

*Built with ❤️ by the Rust community for the Clerk ecosystem*
