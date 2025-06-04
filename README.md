[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# The official community-maintained Clerk SDK for Rust

For more detailed documentation, please reference the below links:

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [Clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

> This SDK is updated frequently to keep up with any changes to the actual Clerk API. If you see anything that needs updating or is not inline with the official Clerk api, please open an issue!

## Environment Setup

To use this SDK, you need to set up your Clerk API secret key as an environment variable. This is the recommended approach to keep your secret keys secure.

1. Create a `.env` file in your project root (you can copy the provided `.env.example` file).
2. Add your Clerk secret key:
   ```
   CLERK_SECRET_KEY=sk_test_your_clerk_secret_key
   ```
3. In your code, load the environment variables using the `dotenv` crate.

> **Important**: Never hardcode your secret keys in your source code.

## Examples

> Check out examples in the `/examples` directory

### Using a traditional http request to a valid clerk endpoint:

```rust
use dotenv::dotenv;
use std::env;
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get the secret key from environment variable
    let secret_key = env::var("CLERK_SECRET_KEY")
        .expect("CLERK_SECRET_KEY environment variable is not set");
        
    let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
    let client = Clerk::new(config);

    let res = client.get(ClerkGetEndpoint::GetUserList).await?;

    Ok(())
}
```

### Using a clerk-rs method:

```rust
use dotenv::dotenv;
use std::env;
use tokio;
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get the secret key from environment variable
    let secret_key = env::var("CLERK_SECRET_KEY")
        .expect("CLERK_SECRET_KEY environment variable is not set");
        
    let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
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
use dotenv::dotenv;
use std::env;

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get the secret key from environment variable
    let secret_key = env::var("CLERK_SECRET_KEY")
        .expect("CLERK_SECRET_KEY environment variable is not set");
        
    HttpServer::new(move || {
        let config = ClerkConfiguration::new(None, None, Some(secret_key.clone()), None);
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
use dotenv::dotenv;
use std::env;

async fn index() -> &'static str {
    "Hello world!"
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get the secret key from environment variable
    let secret_key = env::var("CLERK_SECRET_KEY")
        .expect("CLERK_SECRET_KEY environment variable is not set");
        
    let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
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
use dotenv::dotenv;
use rocket::{
	get, launch, routes,
	serde::{Deserialize, Serialize},
};
use std::env;

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
	// Load environment variables from .env file
	dotenv().ok();
	
	// Get the secret key from environment variable
	let secret_key = env::var("CLERK_SECRET_KEY")
		.expect("CLERK_SECRET_KEY environment variable is not set");
		
	let config = ClerkConfiguration::new(None, None, Some(secret_key), None);
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
use dotenv::dotenv;
use poem::{get, handler, listener::TcpListener, web::Path, EndpointExt, Route, Server};
use std::env;

#[handler]
fn hello(Path(name): Path<String>) -> String {
    format!("hello: {}", name)
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    // Load environment variables from .env file
    dotenv().ok();
    
    // Get the secret key from environment variable
    let secret_key = env::var("CLERK_SECRET_KEY")
        .expect("CLERK_SECRET_KEY environment variable is not set");
        
    let clerk = Clerk::new(ClerkConfiguration::new(
        None,
        None,
        Some(secret_key),
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
- [Have I Been Squatted](https://haveibeensquatted.com)
- Open a PR and add your company here :)

</br>
