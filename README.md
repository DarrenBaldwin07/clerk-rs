[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# 🔐 clerk-rs: Powerful Authentication for Rust

**The official community-maintained Clerk SDK for Rust** - seamlessly integrate Clerk's authentication and user management with your Rust applications.

## ✨ Why clerk-rs?

- **Framework Agnostic** - Works with Actix, Axum, Rocket, Poem, and more
- **Type-Safe API** - Leverage Rust's type system for robust authentication
- **Comprehensive** - Complete access to Clerk's Backend API
- **Performant** - Optimized for speed with async support
- **Community-Driven** - Built by developers, for developers

## 🚀 Getting Started

Add clerk-rs to your project:

```bash
cargo add clerk-rs
```

Enable framework-specific features as needed:

```toml
# In Cargo.toml
clerk-rs = { version = "0.4.1", features = ["actix", "axum", "rocket", "poem"] }
```

## 📚 Examples

### Basic API Access

```rust
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the Clerk client
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    // Fetch users from Clerk
    let users = client.get(ClerkGetEndpoint::GetUserList).await?;
    println!("Users: {:#?}", users);

    Ok(())
}
```

### Framework Integration

<details>
<summary><b>Actix Web</b></summary>

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
        let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
        let clerk = Clerk::new(config);

        App::new()
            .wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
            .route("/protected", web::get().to(protected_route))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```
</details>

<details>
<summary><b>Axum</b></summary>

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
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/protected", get(protected_route))
        .layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), None, true));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
```
</details>

<details>
<summary><b>Rocket</b></summary>

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
    "This route is protected by Clerk authentication!"
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

    rocket::build().mount("/", routes![protected_route]).manage(clerk_config)
}
```
</details>

<details>
<summary><b>Poem</b></summary>

```rust
use clerk_rs::{
    clerk::Clerk,
    validators::{jwks::MemoryCacheJwksProvider, poem::ClerkPoemMiddleware},
    ClerkConfiguration,
};
use poem::{get, handler, listener::TcpListener, web::Path, EndpointExt, Route, Server};

#[handler]
fn protected_route(Path(name): Path<String>) -> String {
    format!("Hello {}, this route is protected by Clerk!", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let clerk = Clerk::new(ClerkConfiguration::new(
        None,
        None,
        Some("your_secret_key".to_owned()),
        None,
    ));
    
    let clerk_poem_middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk.clone()),
        true,
        Some(vec!["/public".to_owned()]), // Exclude routes from auth
    );

    let app = Route::new()
        .at("/protected/:name", get(protected_route))
        .with(clerk_poem_middleware);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```
</details>

Check out more examples in the `/examples` directory!

## 📖 Documentation

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)
- [crates.io documentation](https://docs.rs/clerk-rs)

## 🗺️ Roadmap

- [ ] Support additional HTTP clients (hyper)
- [ ] Support for Tokio and async-std runtimes
- [ ] Optional reqwest blocking client
- [x] Support authorization via __session cookie on same-origin
- [ ] Add validator support for more frameworks

## 🏢 Production Users

<div style="display: flex; align-items: center; gap: 20px;">
  <a href="https://tembo.io" target="_blank">Tembo</a> •
  <a href="https://rezon.ai" target="_blank">Rezon</a> •
  <a href="https://gitar.co" target="_blank">Gitar</a> •
  <a href="https://haveibeensquatted.com" target="_blank">Have I Been Squatted</a>
</div>

> Using clerk-rs in production? Open a PR and add your company here!

## 🤝 Contributing

Contributions are welcome! If you find any issues or have suggestions for improvements, please open an issue or submit a pull request.

> This SDK is updated frequently to keep up with changes to the Clerk API. If you notice anything that needs updating or is not aligned with the official Clerk API, please open an issue!

## 📜 License

clerk-rs is available under the MIT License.