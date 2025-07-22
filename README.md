[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# 🔐 clerk-rs: Supercharge Your Rust Authentication

**The official community-maintained Clerk SDK for Rust** - bringing Clerk's powerful authentication and user management to your Rust applications with ease.

## What is Clerk?

Clerk is a complete user management and authentication solution that handles everything from sign-up to sign-in, session management, and user profiles. With clerk-rs, you can seamlessly integrate Clerk's powerful features into your Rust backend.

## Why Use clerk-rs?

- **Framework Agnostic** - Works with Actix, Axum, Rocket, Poem, and more!
- **Type-Safe API** - Leverage Rust's type system for safer code
- **Comprehensive** - Complete access to Clerk's Backend API
- **Actively Maintained** - Regular updates to stay in sync with Clerk's API

For detailed documentation, check out:

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

> This SDK is updated frequently to keep up with any changes to the actual Clerk API. If you see anything that needs updating or is not inline with the official Clerk API, please open an issue!

## ✨ Key Features

- **Complete API Coverage** - Access all Clerk Backend API endpoints
- **JWT Verification** - Built-in JWT validation for secure session management
- **Framework Integration** - Middleware/guards for popular Rust web frameworks
- **Async Support** - First-class support for async/await
- **Flexible Configuration** - Customize to fit your application's needs

## 🚀 Getting Started

### Installation

Add clerk-rs to your Cargo.toml:

```toml
[dependencies]
clerk-rs = "0.6.1"
```

Enable specific framework support as needed:

```toml
[dependencies]
clerk-rs = { version = "0.6.1", features = ["actix", "axum", "rocket", "poem"] }
```

## 📚 Examples

> Check out more examples in the `/examples` directory

### Using a traditional http request to a valid clerk endpoint:

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

### Protecting a actix-web endpoint with Clerk.dev:

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

### Protecting a axum endpoint with Clerk.dev:

With the `axum` feature enabled:

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

### Protecting a rocket endpoint with Clerk.dev:

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
	let config = ClerkConfiguration::new(None, None, Some("sk_test_F9HM5l3WMTDMdBB0ygcMMAiL37QA6BvXYV1v18Noit".to_string()), None);
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
        Some("sk_test_F9HM5l3WMTDMdBB0ygcMMAiL37QA6BvXYV1v18Noit".to_owned()),
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

## 🛣️ Roadmap

- [ ] Support other HTTP clients alongside the default reqwest client (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via __session cookie on same-origin
- [ ] Expand framework support and optimizations

## ⚡ Production Users

Trusted by innovative companies building with Rust:

- [Tembo](https://tembo.io) - PostgreSQL distribution for AI applications
- [Rezon](https://rezon.ai) - AI-driven data analytics platform
- [Gitar](https://gitar.co) - Software supply chain security
- [Have I Been Squatted](https://haveibeensquatted.com) - Domain typosquatting detection

*Building something cool with clerk-rs? Open a PR and add your company here!*

## 🤝 Contributing

Contributions are welcome! Whether it's bug reports, feature requests, or pull requests, all contributions help make clerk-rs better.

1. Fork the repository
2. Create your feature branch
3. Commit your changes
4. Push to the branch
5. Open a Pull Request

## 📜 License

MIT License