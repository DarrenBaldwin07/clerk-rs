[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

# The official community-maintained Clerk SDK for Rust

`clerk-rs` is a comprehensive Rust SDK for interacting with the [Clerk](https://clerk.dev) authentication API. It provides a type-safe interface to Clerk's backend API and includes middleware support for popular Rust web frameworks.

## Documentation

For more detailed documentation, please reference:

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [Clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

> This SDK is updated frequently to keep up with any changes to the actual Clerk API. If you see anything that needs updating or is not inline with the official Clerk API, please open an issue!

## Features

- Complete Rust implementation of Clerk's Backend API
- First-class middleware support for multiple web frameworks:
  - Actix Web (via `actix` feature)
  - Axum (via `axum` feature)
  - Rocket (via `rocket` feature)
  - Poem (via `poem` feature)
- JWT validation for protected routes
- Type-safe API client for all Clerk endpoints
- Async/await based API
- Session cookie authentication support

## Installation

Add `clerk-rs` to your Cargo.toml:

```toml
[dependencies]
clerk-rs = "0.4.1"
```

If you're using a specific web framework, enable the corresponding feature:

```toml
[dependencies]
clerk-rs = { version = "0.4.1", features = ["actix"] }
# Available features: "actix", "axum", "rocket", "poem"
```

## Examples

> Check out examples in the `/examples` directory

### Using a traditional HTTP request to a valid Clerk endpoint:

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

### Using a clerk-rs method:

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

### Protecting an Actix-web endpoint with Clerk.dev:

With the `actix` feature enabled:

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

### Protecting an Axum endpoint with Clerk.dev:

With the `axum` feature enabled:

```rust
use axum::{routing::get, Router, Extension};
use clerk_rs::{
    clerk::Clerk,
    validators::{authorizer::ClerkJwt, axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

// Public route
async fn index() -> &'static str {
    "Hello world! This is a public route that requires no authentication."
}

// Protected route with authenticated user info
async fn profile(Extension(clerk_jwt): Extension<ClerkJwt>) -> String {
    format!(
        "Hello, {}! This is a protected route.",
        clerk_jwt.sub
    )
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/", get(index))
        .route("/profile", get(profile))
        .layer(ClerkLayer::new(
            MemoryCacheJwksProvider::new(clerk), 
            Some(vec![String::from("/profile")]), // Protected routes
            true, // Enable middleware
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
```

### Protecting a Rocket endpoint with Clerk.dev:

With the `rocket` feature enabled:

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

### Protecting a Poem endpoint with Clerk

With the `poem` feature enabled and poem v3 installed:
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

## API Coverage

This SDK is based on the official [Clerk OpenAPI specification](https://clerk.com/docs/reference/backend-api) and covers:

- User management
- Session handling
- Organization management
- Authentication methods
- JWT and JWKS operations
- Email and phone verification
- And more

## Roadmap

- [ ] Support other http clients along with the default reqwest client (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via \_\_session cookie on same-origin
- [x] Add validator support for axum, rocket, warp

## Production users

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)
- Open a PR and add your company here :)

## License

This project is licensed under the MIT License.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.