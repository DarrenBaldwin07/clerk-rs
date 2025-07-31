# Clerk Rust SDK

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

The official community-maintained Clerk SDK for Rust, providing authentication and user management capabilities for your Rust applications.

## 🚀 Features

- **Complete Clerk API Coverage** - Full support for all Clerk Backend API endpoints
- **Web Framework Integration** - Built-in middleware for popular Rust web frameworks
- **JWT Validation** - Secure token verification with JWKS support
- **Session Management** - Handle user sessions and authentication state
- **Type Safety** - Fully typed API responses and requests

## 📦 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4"
```

### Optional Features

Enable framework-specific features as needed:

```toml
[dependencies]
clerk-rs = { version = "0.4", features = ["actix", "axum", "rocket", "poem"] }
```

Available features:
- `actix` - Actix Web middleware
- `axum` - Axum middleware  
- `rocket` - Rocket guards
- `poem` - Poem middleware
- `rustls-tls` (default) - Use rustls for TLS
- `native-tls` - Use native TLS implementation

## 🔧 Quick Start

### Basic Setup

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(
        None,                                    // jwt_key  
        None,                                    // base_url
        Some("sk_test_your_secret_key".to_string()), // secret_key
        None,                                    // api_version
    );
    
    let clerk = Clerk::new(config);
    
    // Now you can use the clerk client
    Ok(())
}
```

### Making API Calls

#### Using HTTP endpoints directly:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
let client = Clerk::new(config);

// Get list of users
let users = client.get(ClerkGetEndpoint::GetUserList).await?;
```

#### Using SDK methods:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
let client = Clerk::new(config);

// Send an email
Email::create(&client, Some(email_request)).await?;
```

## 🌐 Web Framework Integration

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
            .wrap(ClerkMiddleware::new(
                MemoryCacheJwksProvider::new(clerk.clone()), 
                None, 
                true
            ))
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
use axum::{routing::get, Router};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> &'static str {
    "This route is protected!"
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
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

#[get("/protected")]
fn protected_route(_jwt: ClerkGuard<MemoryCacheJwksProvider>) -> &'static str {
    "This route is protected!"
}

#[launch]
fn rocket() -> _ {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
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
use poem::{get, handler, listener::TcpListener, EndpointExt, Route, Server};

#[handler]
fn protected_route() -> &'static str {
    "This route is protected!"
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let clerk = Clerk::new(config);
    
    let middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        true,
        Some(vec!["/health".to_owned()]), // excluded routes
    );

    let app = Route::new()
        .at("/protected", get(protected_route))
        .with(middleware);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

## 📚 Documentation

- **[Official Clerk API Documentation](https://clerk.com/docs/reference/backend-api)** - Complete API reference
- **[SDK API Documentation](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)** - Rust-specific API docs
- **[Examples Directory](./examples/)** - Complete working examples

## 🛠️ Development

This SDK is actively maintained and frequently updated to stay in sync with the official Clerk API. 

### Running Examples

```bash
# HTTP client example
cargo run --example http

# Framework-specific examples (require features)
cargo run --example actix --features actix
cargo run --example axum --features axum
cargo run --example rocket --features rocket
```

## 🗺️ Roadmap

- [ ] Support additional HTTP clients (hyper, ureq)
- [ ] Async runtime flexibility (tokio, async-std)
- [ ] Optional blocking client support
- [x] Session cookie authorization support
- [ ] Enhanced middleware configuration options

## 🏢 Production Users

Companies using clerk-rs in production:

- [**Tembo**](https://tembo.io) - Postgres development platform
- [**Rezon**](https://rezon.ai) - AI-powered applications  
- [**Gitar**](https://gitar.co) - Developer productivity tools
- [**Have I Been Squatted**](https://haveibeensquatted.com) - Domain security monitoring

*Want to add your company? [Open a PR!](https://github.com/DarrenBaldwin07/clerk-rs/pulls)*

## 🤝 Contributing

We welcome contributions! Please feel free to:

- Report bugs or request features via [GitHub Issues](https://github.com/DarrenBaldwin07/clerk-rs/issues)
- Submit pull requests for improvements
- Help with documentation and examples

## 📄 License

This project is licensed under the MIT License - see the [LICENSE.MD](LICENSE.MD) file for details.
