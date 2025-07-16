[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# clerk-rs: The Official Community-Maintained Clerk SDK for Rust

`clerk-rs` is a Rust SDK for interacting with the [Clerk](https://clerk.com) authentication and user management API. This SDK provides a comprehensive set of tools to integrate Clerk's authentication services into your Rust applications.

## Features

- **Complete API Coverage**: Full implementation of Clerk's Backend API endpoints
- **Type Safety**: Leverage Rust's type system for safe API interactions
- **Framework Integration**: Built-in support for popular Rust web frameworks
  - [Actix Web](#actix-web-integration)
  - [Axum](#axum-integration)
  - [Rocket](#rocket-integration)
  - [Poem](#poem-integration)
- **Authentication Workflows**: Simplified handling of sessions, tokens, and user management
- **Flexible Configuration**: Configure the client to suit your application's needs

## Installation

Add `clerk-rs` to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4.1"
```

To enable support for specific web frameworks, include the relevant feature flag:

```toml
[dependencies]
clerk-rs = { version = "0.4.1", features = ["actix", "axum", "rocket", "poem"] }
```

## Usage

### Basic Configuration

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

// Create a new Clerk client with your secret key
let config = ClerkConfiguration::new(None, None, Some("sk_test_your_key".to_string()), None);
let client = Clerk::new(config);
```

### API Request Examples

#### Direct HTTP Request

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_your_key".to_string()), None);
    let client = Clerk::new(config);

    // Get a list of users
    let users = client.get(ClerkGetEndpoint::GetUserList).await?;
    
    Ok(())
}
```

#### Using Specialized APIs

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_your_key".to_string()), None);
    let client = Clerk::new(config);

    // Create an email
    Email::create(&client, Some(your_clerk_email));
    
    Ok(())
}
```

## Framework Integrations

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
    
    // Initialize middleware
    let clerk_poem_middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk.clone()),
        true,
        // Optionally exclude routes from auth verification
        Some(vec!["/public/route".to_owned()]),
    );

    let app = Route::new()
        .at("/hello/:name", get(hello))
        .with(clerk_poem_middleware); // Add middleware (EndpointExt needs to be in scope)

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

## JWT Access

- In Poem: Use `Data<&ClerkJwt>` or `req.data::<ClerkJwt>()`
- In other frameworks: See respective framework documentation

## Advanced Usage

Check out more detailed examples in the `/examples` directory of this repository.

## Documentation

For detailed documentation, please refer to:

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [clerk-rs API docs](https://docs.rs/clerk-rs)
- [Clerk-rs SDK API reference](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

## Roadmap

- [ ] Support other HTTP clients (hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via `__session` cookie on same-origin
- [x] Framework integrations for actix, axum, rocket, and poem

## Production Users

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)
- Open a PR and add your company here :)

## Contributing

This SDK is updated frequently to keep up with changes to the Clerk API. If you find anything that needs updating or is not aligned with the official Clerk API, please open an issue!

## License

This project is licensed under the MIT License.