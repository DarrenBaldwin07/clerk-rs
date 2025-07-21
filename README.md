<div align="center">

# 🦀 Clerk SDK for Rust

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=for-the-badge&logo=rust&color=orange)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=for-the-badge&logo=rust&color=blue)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=for-the-badge&logo=rust&color=green)](https://docs.rs/clerk-rs)
[![License](https://img.shields.io/crates/l/clerk-rs?style=for-the-badge)](LICENSE.MD)

**The blazingly fast, type-safe authentication SDK for Rust developers**

*Seamlessly integrate [Clerk](https://clerk.com) authentication into your Rust applications with zero boilerplate*

[📚 Documentation](https://docs.rs/clerk-rs) • [🚀 Quick Start](#quick-start) • [💡 Examples](#examples) • [🤝 Contributing](#contributing)

</div>

---

## ✨ Why clerk-rs?

🔐 **Enterprise-grade security** - Built on Clerk's battle-tested authentication platform  
⚡ **Blazingly fast** - Optimized for performance with async/await support  
🎯 **Type-safe** - Full Rust type safety with comprehensive error handling  
🛠️ **Framework agnostic** - Works with Actix, Axum, Rocket, Poem, and more  
📦 **Zero config** - Get started in minutes with sensible defaults  
🔄 **Always up-to-date** - Automatically synced with Clerk's latest API changes

## 🚀 Quick Start

Add `clerk-rs` to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4.1"
tokio = { version = "1.0", features = ["full"] }
```

Get your Clerk secret key from the [Clerk Dashboard](https://dashboard.clerk.com) and start authenticating users:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("sk_test_your_key_here".to_string()), 
        None
    );
    let client = Clerk::new(config);
    
    // You're ready to go! 🎉
    let users = client.users().get_user_list(None, None, None, None, None, None, None).await?;
    println!("Total users: {}", users.len());
    
    Ok(())
}
```

## 📖 Documentation

- 📚 [Crate Documentation](https://docs.rs/clerk-rs) - Complete API reference
- 🔗 [Clerk Backend API](https://clerk.com/docs/reference/backend-api) - Official Clerk API docs
- 📋 [SDK API Reference](https://github.com/DarrenBaldwin07/clerk-rs/blob/main/docs.md) - Detailed method documentation

> 💡 **Stay Updated**: This SDK is actively maintained and automatically synced with Clerk's latest API changes. Found something out of sync? [Open an issue](https://github.com/DarrenBaldwin07/clerk-rs/issues)!

## 💡 Examples

<details>
<summary><strong>🌐 HTTP Client Usage</strong></summary>

Perfect for custom integrations and direct API access:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("sk_test_your_secret_key".to_string()), 
        None
    );
    let client = Clerk::new(config);

    // Direct endpoint access for maximum control
    let user_list = client.get(ClerkGetEndpoint::GetUserList).await?;
    println!("Fetched users: {:?}", user_list);

    Ok(())
}
```
</details>

<details>
<summary><strong>🎯 High-Level API Usage</strong></summary>

Use convenient methods for common operations:

```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, apis::emails_api::Email};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("sk_test_your_secret_key".to_string()), 
        None
    );
    let client = Clerk::new(config);

    // Send emails, manage users, handle organizations
    let result = Email::create(&client, Some("welcome_email_template".to_string())).await;
    println!("Email sent: {:?}", result);

    Ok(())
}
```
</details>

---

## 🛡️ Web Framework Integration

Clerk-rs provides seamless middleware for popular Rust web frameworks:

### 🚀 Actix Web

<details>
<summary>Click to expand Actix Web example</summary>

Enable the `actix` feature in your `Cargo.toml`:

```toml
clerk-rs = { version = "0.4.1", features = ["actix"] }
```

```rust
use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use clerk_rs::{
    clerk::Clerk,
    validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route() -> impl Responder {
    HttpResponse::Ok().json("🔒 You're authenticated! Welcome to the protected area.")
}

async fn public_route() -> impl Responder {
    HttpResponse::Ok().json("🌍 This is a public endpoint, no auth required!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("sk_test_your_secret_key".to_string()), 
        None
    );
    let clerk = Clerk::new(config);

    HttpServer::new(move || {
        App::new()
            .wrap(ClerkMiddleware::new(
                MemoryCacheJwksProvider::new(clerk.clone()), 
                None, 
                true
            ))
            .route("/protected", web::get().to(protected_route))
            .route("/public", web::get().to(public_route))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
```
</details>

### ⚡ Axum

<details>
<summary>Click to expand Axum example</summary>

Enable the `axum` feature in your `Cargo.toml`:

```toml
clerk-rs = { version = "0.4.1", features = ["axum"] }
```

```rust
use axum::{routing::get, Router, Json};
use serde_json::{json, Value};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_handler() -> Json<Value> {
    Json(json!({
        "message": "🚀 Axum + Clerk = Awesome!",
        "authenticated": true
    }))
}

async fn health_check() -> Json<Value> {
    Json(json!({"status": "healthy", "framework": "axum"}))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("sk_test_your_secret_key".to_string()), 
        None
    );
    let clerk = Clerk::new(config);

    let app = Router::new()
        .route("/protected", get(protected_handler))
        .route("/health", get(health_check))
        .layer(ClerkLayer::new(
            MemoryCacheJwksProvider::new(clerk), 
            None, 
            true
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, app).await?;
    Ok(())
}
```
</details>

### 🚀 Rocket

<details>
<summary>Click to expand Rocket example</summary>

Enable the `rocket` feature in your `Cargo.toml`:

```toml
clerk-rs = { version = "0.4.1", features = ["rocket"] }
```

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
    serde_json::Json
};

#[derive(Serialize, Deserialize)]
struct ApiResponse {
    message: String,
    user_authenticated: bool,
}

#[get("/protected")]
fn protected_endpoint(jwt: ClerkGuard<MemoryCacheJwksProvider>) -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "🚀 Welcome to the protected Rocket endpoint!".to_string(),
        user_authenticated: true,
    })
}

#[get("/public")]
fn public_endpoint() -> Json<ApiResponse> {
    Json(ApiResponse {
        message: "🌍 This is a public Rocket endpoint".to_string(),
        user_authenticated: false,
    })
}

#[launch]
fn rocket() -> _ {
    let config = ClerkConfiguration::new(
        None, 
        None, 
        Some("sk_test_your_secret_key".to_string()), 
        None
    );
    let clerk = Clerk::new(config);
    let clerk_config = ClerkGuardConfig::new(
        MemoryCacheJwksProvider::new(clerk),
        None,
        true, // validate_session_cookie
    );

    rocket::build()
        .mount("/", routes![protected_endpoint, public_endpoint])
        .manage(clerk_config)
}
```
</details>

### 📝 Poem

<details>
<summary>Click to expand Poem example</summary>

Enable the `poem` feature in your `Cargo.toml`:

```toml
clerk-rs = { version = "0.4.1", features = ["poem"] }
```

```rust
use clerk_rs::{
    clerk::Clerk,
    validators::{jwks::MemoryCacheJwksProvider, poem::ClerkPoemMiddleware},
    ClerkConfiguration,
};
use poem::{
    get, handler, listener::TcpListener, web::{Path, Data}, 
    EndpointExt, Route, Server, Response, Result as PoemResult
};

#[handler]
fn protected_hello(Path(name): Path<String>) -> PoemResult<String> {
    Ok(format!("🔐 Hello {}! You're authenticated via Poem + Clerk!", name))
}

#[handler]  
fn public_health() -> PoemResult<Response> {
    Ok(Response::builder()
        .content_type("application/json")
        .body(r#"{"status": "healthy", "framework": "poem"}"#))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let clerk = Clerk::new(ClerkConfiguration::new(
        None,
        None,
        Some("sk_test_your_secret_key".to_owned()),
        None,
    ));
    
    let clerk_middleware = ClerkPoemMiddleware::new(
        MemoryCacheJwksProvider::new(clerk.clone()),
        true,
        // Exclude public routes from authentication
        Some(vec!["/health".to_owned()]),
    );

    let app = Route::new()
        .at("/hello/:name", get(protected_hello))
        .at("/health", get(public_health))
        .with(clerk_middleware);

    println!("🚀 Poem server running on http://localhost:3000");
    Server::new(TcpListener::bind("0.0.0.0:3000"))
        .run(app)
        .await?;
    
    Ok(())
}
```

> 💡 **Pro tip**: Access JWT data using `Data<&ClerkJwt>` or `req.data::<ClerkJwt>()` in your handlers.

</details>

---

## 🗺️ Roadmap

- [ ] 🌐 Support additional HTTP clients (hyper, etc.)
- [ ] ⚡ Multiple async runtimes (tokio, async-std)
- [ ] 🔄 Optional blocking client support
- [x] 🍪 Session cookie authorization support
- [ ] 🚀 Additional framework integrations (warp, tide)
- [ ] 📊 Built-in metrics and observability
- [ ] 🔧 Advanced caching strategies

## 🏢 Production Users

These companies trust clerk-rs in production:

<table>
<tr>
<td align="center">
<a href="https://tembo.io"><strong>Tembo</strong></a><br>
<em>Postgres in the cloud</em>
</td>
<td align="center">
<a href="https://rezon.ai"><strong>Rezon</strong></a><br>
<em>AI-powered solutions</em>
</td>
</tr>
<tr>
<td align="center">
<a href="https://gitar.co"><strong>Gitar</strong></a><br>
<em>Code collaboration</em>
</td>
<td align="center">
<a href="https://haveibeensquatted.com"><strong>Have I Been Squatted</strong></a><br>
<em>Domain security</em>
</td>
</tr>
</table>

**Using clerk-rs in production?** [Open a PR](https://github.com/DarrenBaldwin07/clerk-rs/pulls) and add your company! 🚀

---

## 🤝 Contributing

We love contributions! Whether it's:

- 🐛 **Bug reports** - Help us squash those bugs
- ✨ **Feature requests** - Share your ideas  
- 📝 **Documentation** - Make it clearer for everyone
- 🔧 **Code contributions** - Submit that PR!

Check out our [issues](https://github.com/DarrenBaldwin07/clerk-rs/issues) to get started!

## 📄 License

Licensed under the [MIT License](LICENSE.MD) - see the file for details.

---

<div align="center">

**Built with ❤️ by the Rust community**

[⭐ Star us on GitHub](https://github.com/DarrenBaldwin07/clerk-rs) • [📖 Read the docs](https://docs.rs/clerk-rs) • [💬 Join discussions](https://github.com/DarrenBaldwin07/clerk-rs/discussions)

*Made with 🦀 and powered by [Clerk](https://clerk.com)*

</div>
