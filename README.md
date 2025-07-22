<div align="center">

# 🔐 clerk-rs

### The official community-maintained Clerk SDK for Rust

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)
[![Rust](https://img.shields.io/badge/rust-stable-orange?style=flat-square)](https://www.rust-lang.org/)
[![License](https://img.shields.io/crates/l/clerk-rs?style=flat-square)](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/LICENSE)

**Seamless authentication and user management for your Rust applications**

[Installation](#installation) • [Quick Start](#quick-start) • [Examples](#examples) • [Documentation](#documentation) • [Production Users](#production-users)

</div>

## ✨ Features

- 🛡️ **Complete Authentication** - Full implementation of Clerk's authentication API
- 🚀 **Framework Support** - Integrates with Actix, Axum, Rocket, and Poem
- 🔄 **JWT Verification** - Built-in JWT validation and session management
- 🔌 **Flexible** - Use as a standalone client or integrate with your favorite web framework
- 🧩 **Modular Design** - Only include the features you need with cargo features

## 🔥 Installation

Add clerk-rs to your Cargo.toml:

```toml
[dependencies]
clerk-rs = "0.8.0"
```

Or use cargo-add from the command line:

```bash
cargo add clerk-rs
```

Enable specific framework support with features:

```toml
[dependencies]
clerk-rs = { version = "0.8.0", features = ["actix", "axum"] }
```

## 🚀 Quick Start

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

// Initialize Clerk with your secret key
let config = ClerkConfiguration::new(None, None, Some("sk_test_your_api_key".to_string()), None);
let clerk = Clerk::new(config);

// Now you can use the clerk client to access the API
// Example: Get a list of users
let users = clerk.users().get_user_list().await?;
```

## 📚 Documentation

For complete documentation, please reference:

- [Official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api)
- [Clerk-rs SDK API docs](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md)

> This SDK is updated frequently to keep up with any changes to the actual Clerk API. If you see anything that needs updating or is not inline with the official Clerk API, please open an issue!

## 💻 Examples

> 📁 **More examples** in the [`/examples`](https://github.com/DarrenBaldwin07/clerk-rs/tree/main/examples) directory

<details>
<summary><b>🔗 Basic API Usage</b></summary>

#### Using a direct HTTP request

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

#### Using clerk-rs methods

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

</details>

<details>
<summary><b>🛡️ Web Framework Integration</b></summary>

### Actix Web Integration

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

### Axum Integration

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

### Rocket Integration

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

### Poem Integration

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
</details>

## Roadmap

- [ ] Support other http clients along with the default reqwest client (like hyper)
- [ ] Tokio and async-std async runtimes for hyper clients
- [ ] Optional reqwest blocking client
- [x] Support authorization via \_\_session cookie on same-origin
- [ ] Add validator support for axum, rocket, warp

## 🏗️ Production Users

clerk-rs is trusted by these companies in production:

<div align="center">

| [![Tembo](https://img.shields.io/badge/Tembo-PostgreSQL%20Platform-orange?style=for-the-badge&logo=data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAABQAAAAUCAMAAAC6V+0/AAABS2lUWHRYTUw6Y29tLmFkb2JlLnhtcAAAAAAAPD94cGFja2V0IGJlZ2luPSLvu78iIGlkPSJXNU0wTXBDZWhpSHpyZVN6TlRjemtjOWQiPz4KPHg6eG1wbWV0YSB4bWxuczp4PSJhZG9iZTpuczptZXRhLyIgeDp4bXB0az0iQWRvYmUgWE1QIENvcmUgNS42LWMxNDAgNzkuMTYwNDUxLCAyMDE3LzA1LzA2LTAxOjA4OjIxICAgICAgICAiPgogPHJkZjpSREYgeG1sbnM6cmRmPSJodHRwOi8vd3d3LnczLm9yZy8xOTk5LzAyLzIyLXJkZi1zeW50YXgtbnMjIj4KICA8cmRmOkRlc2NyaXB0aW9uIHJkZjphYm91dD0iIi8+CiA8L3JkZjpSREY+CjwveDp4bXBtZXRhPgo8P3hwYWNrZXQgZW5kPSJyIj8+LUNEtwAAAARnQU1BAACxjwv8YQUAAAABc1JHQgCuzhzpAAAAPFBMVEX///8AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAADMzMzm5uYNzu0SAAAAFHRSTlMABAgMFBgcICQsMDQ8d4ePk5erhCSoDxEAAACbSURBVBjTfdBLEsQgCARQPgrGRPH+5x0kJlqxZvGavwQBBM8TbUQW0e/F2GMKqPFEUAA8KZhZ9KAQmFO4VQjMOaIoySmkKEkpvCgIKc6zuFEQUlDiRDtOVBQpBIGAoJhJbMXgIpCZdBZ2eCvM8kvpLEZyKTLRyQY8kaYaGPQKc3Qaj/RHjaTelzYkx7aZkZ6B5BL+rL8/Z+4LnVsI3WP3CnAAAAAASUVORK5CYII=)](https://tembo.io) | [![Rezon](https://img.shields.io/badge/Rezon-AI%20Solutions-blue?style=for-the-badge&logo=data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHZpZXdCb3g9IjAgMCAyNCAyNCI+PHBhdGggZD0iTTEyIDJjNS41MiAwIDEwIDQuNDggMTAgMTBzLTQuNDggMTAtMTAgMTBTMiAxNy41MiAyIDEyIDYuNDggMiAxMiAyek0xMCAyMHYtNmgtMnYtNGgyVjZoNnY0aC00djJoNHY4aC02eiIgZmlsbD0id2hpdGUiLz48L3N2Zz4=)](https://rezon.ai) |
| [![Gitar](https://img.shields.io/badge/Gitar-Code%20Collaboration-success?style=for-the-badge&logo=git)](https://gitar.co) | [![Have I Been Squatted](https://img.shields.io/badge/Have%20I%20Been%20Squatted-Security-red?style=for-the-badge&logo=shield)](https://haveibeensquatted.com) |

</div>

*Using clerk-rs in production? [Open a PR](https://github.com/DarrenBaldwin07/clerk-rs/pulls) to add your company here!*

## 💪 Contributing

Contributions are welcome and appreciated! Here's how you can help:

1. **Fork the repository** and clone it locally
2. **Create a new branch** for your feature or bugfix
3. **Make your changes** and test thoroughly
4. **Submit a pull request** with a clear description of the changes

Please make sure your code follows the existing style and that all tests pass before submitting a PR.

## 👏 Acknowledgements

This project exists thanks to all the contributors who participate in this community-maintained effort.

## 📜 License

This project is MIT licensed. See the [LICENSE](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/LICENSE) file for details.
