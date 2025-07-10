[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# clerk-rs

**The Official Community-Maintained Clerk SDK for Rust**

![Clerk + Rust](https://img.shields.io/badge/Clerk%20%2B%20Rust-Auth%20Made%20Easy-8A2BE2?style=flat-square)

A powerful, idiomatic Rust SDK for [Clerk](https://clerk.com/) authentication and user management. This community-maintained library provides:

- **Seamless integration** with Clerk's Backend API
- **First-class support** for popular Rust web frameworks (Actix Web, Axum, Rocket, Poem)
- **Minimal boilerplate** for adding secure auth to your Rust applications

## ✨ Features

- **Complete API coverage** of Clerk's Backend API
- **Framework-specific integrations**:
  - 🌐 Actix Web
  - 🚀 Axum
  - 🔥 Rocket
  - 📝 Poem
- **Robust security**:
  - JWT validation and session management
  - Memory-cached JWKS provider for efficient authentication
  - Session cookie support
- **Developer experience**:
  - Type-safe API client
  - Comprehensive error handling
  - Configurable authentication options

## 📚 Documentation

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)
- [Examples directory](/examples)
- [API Reference on docs.rs](https://docs.rs/clerk-rs)

> **Note:** This SDK is actively maintained to stay in sync with the official Clerk API. If you find any discrepancies or missing features, please [open an issue](https://github.com/DarrenBaldwin07/clerk-rs/issues)!

## 🚀 Quick Start

Get started with clerk-rs in three simple steps:

1. **Sign up for Clerk** at [clerk.com](https://clerk.com/) and create an application to obtain your API keys
2. **Add clerk-rs** to your Rust project (see Installation section below)
3. **Integrate authentication** using the examples for your preferred web framework

## 📦 Installation

Add clerk-rs to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.1"  # Always check crates.io for the latest version
```

To enable framework-specific features:

```toml
[dependencies]
clerk-rs = { version = "0.1", features = ["actix", "axum", "rocket", "poem"] }
```

## 💻 Usage Examples

### Basic HTTP Request

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Clerk with your secret key
    // In production, load this from environment variables
    let config = ClerkConfiguration::new(
        None,                              // frontend API key (optional)
        None,                              // JWT key (optional)
        Some("sk_test_key".to_string()),  // secret key
        None,                              // custom API URL (optional)
    );
    let client = Clerk::new(config);

    // Fetch user list from Clerk API
    let users = client.get(ClerkGetEndpoint::GetUserList).await?;
    println!("Retrieved {} users", users.len());
    
    Ok(())
}
```

### Using High-Level API Methods

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize Clerk client
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);
    
    // Create email data
    let email_data = json!({
        "from_email_name": "support",
        "subject": "Welcome to our service",
        "body": "Thanks for signing up!",
        "email_address_id": "email_address_id_value"
    });
    
    // Send email using Clerk's Email API
    let response = Email::create(&client, Some(email_data)).await?;
    println!("Email sent successfully with ID: {}", response["id"]);

    Ok(())
}
```

### Protecting Routes in Actix Web

```rust
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

// This route is protected by Clerk authentication
async fn protected_route() -> impl Responder {
    HttpResponse::Ok().body("You are authenticated!")
}

// Public route (no authentication required)
async fn public_route() -> impl Responder {
    HttpResponse::Ok().body("Public route - no auth required")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables in production
    let secret_key = std::env::var("CLERK_SECRET_KEY")
        .unwrap_or_else(|_| "sk_test_key".to_string());
    
    HttpServer::new(move || {
        // Create Clerk client and middleware
        let config = ClerkConfiguration::new(None, None, Some(secret_key.clone()), None);
        let clerk = Clerk::new(config);
        let clerk_middleware = ClerkMiddleware::new(
            MemoryCacheJwksProvider::new(clerk),
            None,         // No custom JWT verification
            true,         // Validate session cookies
        );

        App::new()
            // Apply Clerk middleware to all routes
            .wrap(clerk_middleware)
            // Protected route requires authentication
            .route("/protected", web::get().to(protected_route))
            // Add public routes before authentication middleware
            .service(
                web::scope("/public")
                    .wrap(actix_web::middleware::Condition::new(false, clerk_middleware.clone()))
                    .route("", web::get().to(public_route))
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

### Protecting Routes in Axum

```rust
use axum::{routing::get, Router, extract::State, response::IntoResponse};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider, jwt::ClerkJwt},
    ClerkConfiguration,
};

// Protected route - requires authentication
async fn protected_route(State(user_jwt): State<ClerkJwt>) -> impl IntoResponse {
    // Access user information from the JWT
    let user_id = user_jwt.claims.sub;
    format!("Hello authenticated user: {}", user_id)
}

// Public route - no authentication required
async fn public_route() -> &'static str {
    "Public route - accessible to everyone"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables in production
    let secret_key = std::env::var("CLERK_SECRET_KEY")
        .unwrap_or_else(|_| "sk_test_key".to_string());
    
    // Create Clerk client
    let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
    let clerk = Clerk::new(config);
    
    // Create JWT validator
    let jwks_provider = MemoryCacheJwksProvider::new(clerk);
    
    // Build authenticated routes
    let protected_routes = Router::new()
        .route("/protected", get(protected_route))
        .layer(ClerkLayer::new(jwks_provider.clone(), None, true));
        
    // Build public routes
    let public_routes = Router::new()
        .route("/public", get(public_route));
        
    // Combine routes
    let app = Router::new()
        .merge(protected_routes)
        .merge(public_routes);

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    println!("Server listening on http://0.0.0.0:8080");
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

// This route requires authentication
#[get("/protected")]
fn protected_route(jwt: ClerkGuard<MemoryCacheJwksProvider>) -> &'static str {
    // Access JWT data via jwt.claims if needed
    "You are authenticated!"
}

// Public route - no auth required
#[get("/public")]
fn public_route() -> &'static str {
    "This is a public route - no auth required"
}

#[launch]
fn rocket() -> _ {
    // Load from environment in production
    let secret_key = std::env::var("CLERK_SECRET_KEY")
        .unwrap_or_else(|_| "sk_test_key".to_string());
        
    let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
    let clerk = Clerk::new(config);
    let clerk_config = ClerkGuardConfig::new(
        MemoryCacheJwksProvider::new(clerk),
        None,               // No custom JWT verification
        true,               // Validate session cookie
    );

    rocket::build()
        .mount("/", routes![protected_route, public_route])
        .manage(clerk_config)
}
```

### Protecting Routes in Poem

```rust
use clerk_rs::{
    clerk::Clerk,
    validators::{jwks::MemoryCacheJwksProvider, poem::ClerkPoemMiddleware, jwt::ClerkJwt},
    ClerkConfiguration,
};
use poem::{
    get, handler, listener::TcpListener, web::{Path, Data}, 
    EndpointExt, Route, Server, Response, middleware::AddData
};

// Protected route - requires authentication
#[handler]
fn protected(Data(jwt): Data<&ClerkJwt>) -> String {
    // Access user information from the JWT
    format!("Hello authenticated user: {}", jwt.claims.sub)
}

// Public route - no authentication required
#[handler]
fn public() -> &'static str {
    "This is a public route - no auth needed"
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Load from environment in production
    let secret_key = std::env::var("CLERK_SECRET_KEY")
        .unwrap_or_else(|_| "sk_test_key".to_string());
        
    let clerk = Clerk::new(ClerkConfiguration::new(
        None,
        None,
        Some(secret_key),
        None,
    ));
    
    // Initialize middleware
    let clerk_middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk.clone()),
        true,                // Validate session cookies
        Some(vec!["/public"]), // Exclude these routes from auth
    );

    // Build routes
    let app = Route::new()
        .at("/protected", get(protected))
        .at("/public", get(public))
        .with(clerk_middleware); // Add middleware (EndpointExt needs to be in scope)

    // Start server
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

The JWT can be accessed using `Data<&ClerkJwt>` (or `req.data::<ClerkJwt>()`). This gives you access to the decoded JWT payload containing user information and claims.

## 🗺️ Roadmap

- [ ] Support other HTTP clients along with the default reqwest client (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via \_\_session cookie on same-origin
- [x] Add validator support for axum, rocket, poem
- [ ] Improved error handling and custom error types
- [ ] Enhanced documentation with more examples
- [ ] Frontend integration examples for popular Rust WASM frameworks

## 🏢 Production Users

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)

*Using clerk-rs in production? [Open a PR](https://github.com/DarrenBaldwin07/clerk-rs/pulls) to add your company to this list!*

## 👥 Contributing

Contributions are welcome! Please feel free to submit a Pull Request. Check out the roadmap for areas that need attention or propose your own improvements.

## 🆘 Support

If you encounter any issues or have questions, please [open an issue](https://github.com/DarrenBaldwin07/clerk-rs/issues) on GitHub.

## 📄 License

This project is licensed under the [MIT License](LICENSE).