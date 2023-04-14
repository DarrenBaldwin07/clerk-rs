# An unofficial clerk.dev SDK for rust
> Note: This SDK is updated frequently to keep up with any changes to the actual Clerk API. However, if you  This SDK is by no means affiliated with Clerk.dev.

A unofficial clerk.dev SDK. For more detailed documentation, please reference the clerk docs: https://clerk.com/docs/reference/backend-api

## Example
> More examples in a `/examples` directory coming soon...

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
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint, apis::emails_api::Email};

async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);

    Email::create(&client, Some(your_clerk_email));

    Ok(())
}
```

## Roadmap
 - [ ] Support other http clients along with the default reqwest client (like hyper)
 - [ ] Tokio and async-std async runtimes for hyper clients

</br>

[![name](https://user-images.githubusercontent.com/68653294/232151991-265606f7-e31c-4c0e-8659-4e339a0b99a0.svg)](https://cincinnati.ventures)
