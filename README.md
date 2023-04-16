# An unofficial clerk.dev SDK for rust
> Note: This SDK is updated frequently to keep up with any changes to the actual Clerk API. If you see anything that needs updated or is not inline with the offical Clerk api, open an issue!

A unofficial clerk.dev SDK. For more detailed documentation, please reference the clerk docs: https://clerk.com/docs/reference/backend-api

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
                .wrap(ClerkMiddleware::new(config))
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
 - [ ] Support authorization via __session cookie on same-origin
 - [ ] Add validator support for axum, rocket, warp


> Note: This SDK is completely maintained by the Rust community and is by no means affiliated with Clerk.dev.

</br>

[![name](https://user-images.githubusercontent.com/68653294/232151991-265606f7-e31c-4c0e-8659-4e339a0b99a0.svg)](https://cincinnati.ventures)
