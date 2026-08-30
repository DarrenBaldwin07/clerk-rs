[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

# clerk-rs

The community-maintained Rust SDK for the [Clerk Backend API](https://clerk.com/docs/reference/backend-api), plus request-authentication middleware for Actix Web, Axum, Rocket, and Poem.

Version `0.5` targets Clerk Backend API version `2026-05-12`. The generated API and model modules come from Clerk's official dated OpenAPI specification and use `https://api.clerk.com/v1` by default.

## Calling the Backend API

```rust,no_run
use clerk_rs::{apis::users_api, Clerk};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clerk = Clerk::from_secret_key("your_secret_key");
    let user = users_api::get_user(&clerk.config, "user_id").await?;

    println!("{}", user.id);
    Ok(())
}
```

Every generated function accepts `&ClerkConfiguration` as its first argument:

```rust,no_run
use clerk_rs::{apis::organizations_api, ClerkConfiguration};

# async fn example() -> Result<(), Box<dyn std::error::Error>> {
let config = ClerkConfiguration::new("your_secret_key");
let organization = organizations_api::get_organization(
    &config,
    "organization_id",
    None,
    None,
).await?;
# Ok(())
# }
```

See the generated [`docs`](docs/) directory for all API groups, request types, and response models.

## Authentication middleware

Enable the integration for your web framework:

```toml
[dependencies]
clerk-rs = { version = "0.5", features = ["axum"] }
```

### Protecting an Actix Web endpoint

Enable the `actix` feature, then wrap the application with `ClerkMiddleware`:

```rust,no_run
use actix_web::{web, App, HttpServer, Responder};
use clerk_rs::{
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    Clerk,
};

async fn index() -> impl Responder {
    "Hello world!"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let clerk = Clerk::from_secret_key("your_secret_key");

        App::new()
            .wrap(ClerkMiddleware::new(
                MemoryCacheJwksProvider::new(clerk),
                None,
                true,
            ))
            .route("/index", web::get().to(index))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```

See the complete [Actix Web example](examples/actix.rs).

### Protecting an Axum endpoint

Enable the `axum` feature, then add `ClerkLayer` to the router. The authenticated JWT is available through Axum's `Extension` extractor on protected routes:

```rust,no_run
use axum::{routing::get, Extension, Router};
use clerk_rs::{
    validators::{
        authorizer::ClerkJwt,
        axum::ClerkLayer,
        jwks::MemoryCacheJwksProvider,
    },
    Clerk,
};

async fn profile(Extension(jwt): Extension<ClerkJwt>) -> String {
    format!("Hello, {}!", jwt.sub)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let clerk = Clerk::from_secret_key("your_secret_key");

    let app = Router::new()
        .route("/profile", get(profile))
        .layer(ClerkLayer::new(
            MemoryCacheJwksProvider::new(clerk),
            Some(vec!["/profile".to_owned()]),
            true,
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await?;
    axum::serve(listener, app).await
}
```

The route list passed to `ClerkLayer::new` selects protected routes. Passing `None` protects every route. See the complete [Axum example](examples/axum.rs).

### Protecting a Rocket endpoint

Enable the `rocket` feature, register `ClerkGuardConfig`, and add `ClerkGuard` to protected route handlers:

```rust,no_run
use clerk_rs::{
    validators::{
        jwks::MemoryCacheJwksProvider,
        rocket::{ClerkGuard, ClerkGuardConfig},
    },
    Clerk,
};
use rocket::{get, launch, routes};

#[get("/")]
fn index(_jwt: ClerkGuard<MemoryCacheJwksProvider>) -> &'static str {
    "Hello world!"
}

#[launch]
fn rocket() -> _ {
    let clerk = Clerk::from_secret_key("your_secret_key");
    let guard_config = ClerkGuardConfig::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true,
    );

    rocket::build()
        .mount("/", routes![index])
        .manage(guard_config)
}
```

See the complete [Rocket example](examples/rocket.rs).

### Protecting a Poem endpoint

Enable the `poem` feature, then apply `ClerkPoemMiddleware` to the route:

```rust,no_run
use clerk_rs::{
    validators::{jwks::MemoryCacheJwksProvider, poem::ClerkPoemMiddleware},
    Clerk,
};
use poem::{get, handler, listener::TcpListener, EndpointExt, Route, Server};

#[handler]
fn hello() -> &'static str {
    "Hello world!"
}

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let clerk = Clerk::from_secret_key("your_secret_key");
    let middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk),
        true,
        None,
    );

    let app = Route::new().at("/", get(hello)).with(middleware);

    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await
}
```

The authenticated JWT is available as `Data<&ClerkJwt>` or through `req.data::<ClerkJwt>()`. The final constructor argument optionally lists routes excluded from authentication.

Live validator integration tests for all four frameworks are documented in [`tests/README.md`](tests/README.md). They are ignored during normal test runs and require credentials for a development Clerk instance.

## API versioning

`ClerkConfiguration::default()` and `ClerkConfiguration::new()` send:

```text
Clerk-API-Version: 2026-05-12
```

An older supported Clerk version can be selected explicitly:

```rust
use clerk_rs::ClerkConfiguration;

let config = ClerkConfiguration::new("your_secret_key")
    .with_api_version("2025-11-10");
```

The Rust types still describe `2026-05-12`; selecting an older wire version does not change compile-time types.

## Regenerating the SDK

The repository vendors Clerk's exact specification at [`openapi/clerk-bapi-2026-05-12.yml`](openapi/clerk-bapi-2026-05-12.yml) and pins OpenAPI Generator in [`openapitools.json`](openapitools.json).

Install `@openapitools/openapi-generator-cli`, then run:

```sh
./scripts/generate-openapi.sh
```

The script verifies the specification checksum, regenerates `src/apis`, `src/models`, and `docs`, preserves the hand-written Clerk configuration, and formats the result.

## TLS and framework features

- `rustls-tls` (default)
- `native-tls`
- `actix`
- `axum`
- `rocket`
- `poem`

## Production users

- [Tembo](https://tembo.io)
- [Rezon](https://rezon.ai)
- [Gitar](https://gitar.co)
- [Have I Been Squatted](https://haveibeensquatted.com)
- Open a PR and add your company here :)

## License

MIT
