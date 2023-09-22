<a href="https://crates.io/crates/clerk-rs">
    <img src="https://img.shields.io/crates/v/clerk-rs.svg?style=flat-square" alt="crates.io" />
</a>
<a href="https://crates.io/crates/clerk-rs">
    <img src="https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square"
        alt="download count badge" />
</a>

# An unofficial clerk.dev SDK for rust

For more detailed documentation, please reference the below links:

- [Offical Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [Clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

> This SDK is updated frequently to keep up with any changes to the actual Clerk API. If you see anything that needs updated or is not inline with the official Clerk api, please open an issue!

## Example

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

```rust
use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{ClerkConfiguration, validators::actix::ClerkMiddleware};

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
        App::new().service(
            web::scope("/app")
                .wrap(ClerkMiddleware::new(config, None, false))
                .route("/index", web::get().to(index)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

## Roadmap

- [ ] Support other http clients along with the default reqwest client (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [ ] Support authorization via \_\_session cookie on same-origin
- [ ] Add validator support for axum, rocket, warp

# Production users

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)

> Note: This SDK is completely maintained by the Rust community and is by no means affiliated with Clerk.dev.

</br>
