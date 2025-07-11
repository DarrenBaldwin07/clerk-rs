[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# clerk-rs: The Official Community-Maintained Clerk SDK for Rust

A comprehensive Rust SDK for integrating [Clerk](https://clerk.com) authentication and user management into your Rust applications. This SDK supports multiple web frameworks including Actix Web, Axum, Rocket, and Poem.

## Features

- Complete Clerk API support with strongly typed interfaces
- Authentication middleware for popular Rust web frameworks
- JWT validation with configurable caching strategies
- Session management including cookie-based authentication
- Async/await support with Tokio runtime

## Installation

Add `clerk-rs` to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4.1"
```

To enable specific web framework support, include the corresponding feature:

```toml
# For Actix Web support
clerk-rs = { version = "0.4.1", features = ["actix"] }

# For Axum support
clerk-rs = { version = "0.4.1", features = ["axum"] }

# For Rocket support
clerk-rs = { version = "0.4.1", features = ["rocket"] }

# For Poem support
clerk-rs = { version = "0.4.1", features = ["poem"] }
```

## Basic Usage

### Configuration

Create a Clerk client with your API key:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

// Initialize Clerk with your secret key
let config = ClerkConfiguration::new(
    None, // frontend API key (optional)
    None, // API version (optional)
    Some("sk_test_your_secret_key".to_string()), // backend API key
    None, // base URL (optional)
);
let client = Clerk::new(config);
```

### Making API Requests

#### Using Direct HTTP Methods

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_your_secret_key".to_string()), None);
    let client = Clerk::new(config);

    // Get a list of users
    let users = client.get(ClerkGetEndpoint::GetUserList).await?;
    
    println!("Users: {:?}", users);
    
    Ok(())
}
```

#### Using Typed API Methods

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_your_secret_key".to_string()), None);
    let client = Clerk::new(config);

    // Create a new email
    let email_data = /* your email data */;
    let result = Email::create(&client, Some(email_data)).await?;
    
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
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    HttpServer::new(|| {
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
use rocket::{get, launch, routes, serde::{Deserialize, Serialize}};

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
        Some("your_secret_key".to_string()),
        None,
    ));
    
    // Initialize middleware with optional route exclusions
    let clerk_poem_middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk.clone()),
        true,
        Some(vec!["/api/docs".to_owned()]),
    );

    let app = Route::new()
        .at("/hello/:name", get(hello))
        .with(clerk_poem_middleware);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

## Advanced Usage

### JWT Validation

Clerk-rs provides JWT validation with configurable caching strategies:

```rust
use clerk_rs::{
    clerk::Clerk,
    validators::jwks::{MemoryCacheJwksProvider, JwksProvider},
    ClerkConfiguration,
};

async fn validate_token(token: &str) -> Result<bool, Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);
    
    let jwks_provider = MemoryCacheJwksProvider::new(clerk);
    let result = jwks_provider.validate_token(token, None, true).await?;
    
    Ok(result.is_some())
}
```

## Documentation

For more detailed documentation, please refer to:

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [Clerk-rs SDK API docs](https://docs.rs/clerk-rs)

## Examples

Check out the `/examples` directory in the repository for complete working examples.

## Roadmap

- [ ] Support other HTTP clients along with the default reqwest client (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via \_\_session cookie on same-origin
- [x] Add validator support for axum, rocket, poem frameworks
- [ ] Expanded test coverage

## Production Users

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)
- Open a PR and add your company here :)

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License.