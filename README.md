<a href="https://crates.io/crates/clerk-rs">
    <img src="https://img.shields.io/crates/v/clerk-rs.svg?style=flat-square" alt="crates.io" />
</a>
<a href="https://crates.io/crates/clerk-rs">
    <img src="https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square"
        alt="download count badge" />
</a>

# A official community-maintained Clerk SDK for Rust

For more detailed documentation, please reference the below links:

- [Offical Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [Clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

> This SDK is updated frequently to keep up with any changes to the actual Clerk API. If you see anything that needs updated or is not inline with the official Clerk api, please open an issue!

## Examples

> Checkout examples in the `/examples` directory

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
use clerk_rs::{validators::actix::ClerkMiddleware, ClerkConfiguration};

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
        App::new()
            .wrap(ClerkMiddleware::new(config, None, true))
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
use clerk_rs::validators::axum::ClerkLayer;
use clerk_rs::ClerkConfiguration;
use std::net::SocketAddr;

async fn index() -> &'static str {
    "Hello world!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let config: ClerkConfiguration = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let app = Router::new().route("/index", get(index)).layer(ClerkLayer::new(config, None, true));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await
}
```

## Roadmap

- [ ] Support other http clients along with the default reqwest client (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via \_\_session cookie on same-origin
- [ ] Add validator support for axum, rocket, warp

# Production users

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- Open a PR and add your company here :)

</br>
