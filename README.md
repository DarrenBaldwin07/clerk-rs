<div align="center">

# 🔐 clerk-rs

**Seamless Authentication for Rust Web Applications**

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=flat-square)](https://opensource.org/licenses/MIT)

</div>

## The official community-maintained Clerk SDK for Rust

**clerk-rs** brings the power of [Clerk's](https://clerk.com) authentication and user management to the Rust ecosystem. Integrate secure, feature-rich auth into your Rust applications with support for **actix-web**, **axum**, **rocket**, and **poem** frameworks.

### Why clerk-rs?

- 🚀 **Simple Integration** - Add authentication to your Rust web apps in minutes, not days
- 🛡️ **Security First** - Built on Clerk's proven authentication platform
- 🧩 **Framework Agnostic** - Works with popular Rust web frameworks
- 🔄 **Up-to-Date** - Maintained in sync with Clerk's official API

### Resources

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [Clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

> This SDK is updated frequently to keep up with any changes to the actual Clerk API. If you see anything that needs updating or is not inline with the official Clerk API, please open an issue!

## ✨ Features

- 📦 **Complete API Coverage** - Access all Clerk authentication endpoints
- 🔒 **JWT Verification** - Validate session tokens with ease
- 🔗 **Framework Integration** - Middleware/guards for popular Rust web frameworks
- 🧠 **Smart Caching** - Efficient JWKS caching to minimize API calls
- ⚙️ **Flexible Configuration** - Customize to fit your application's needs

## 🚀 Getting Started

```bash
cargo add clerk-rs
# Add optional features based on your web framework
cargo add clerk-rs --features actix  # For actix-web support
cargo add clerk-rs --features axum   # For axum support
cargo add clerk-rs --features rocket # For rocket support
cargo add clerk-rs --features poem   # For poem support
```

## 📚 Examples

> Explore complete examples in the `/examples` directory

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

## 🗺️ Roadmap

- [ ] Support alternative HTTP clients alongside reqwest (like hyper)
- [ ] Support for both Tokio and async-std runtimes with hyper clients
- [ ] Add optional reqwest blocking client for synchronous workflows
- [x] Support authorization via \_\_session cookie on same-origin requests
- [x] Add validator support for axum, rocket, and poem
- [ ] Add validator support for warp
- [ ] Enhanced error handling and diagnostics
- [ ] Comprehensive test coverage across all features

## 🏢 Trusted in Production

These organizations trust clerk-rs in production environments:

<div align="center">

| [<img src="https://tembo.io/tembo.svg" width="120px" />](https://tembo.io) | [<img src="https://rezon.ai/logo.svg" width="120px" />](https://rezon.ai) | [<img src="https://gitar.co/logo.png" width="120px" />](https://gitar.co) |
|:---:|:---:|:---:|
| Tembo | Rezon | Gitar |

</div>

And many more, including [Have I Been Squatted](https://haveibeensquatted.com)!

> **Using clerk-rs in production?** Open a PR to add your company to this list!

## 🤝 Contributing

Contributions are welcome! Whether it's bug reports, feature requests, or pull requests, all contributions help make clerk-rs better for everyone.

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.
