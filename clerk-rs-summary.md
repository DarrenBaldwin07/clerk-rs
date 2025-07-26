# Clerk-rs: Official Community Rust SDK for Clerk Authentication

## Overview

**Clerk-rs** is the official community-maintained Rust SDK for [Clerk](https://clerk.com), a complete user management platform. This SDK provides Rust developers with seamless access to Clerk's comprehensive authentication and user management APIs, enabling them to build secure applications with features like user registration, authentication, organization management, and session handling.

## What is Clerk?

Clerk is a modern authentication and user management service that provides:
- User authentication (email/password, social logins, passwordless)
- User profile management
- Organizations and role-based access control
- Session management
- Multi-factor authentication (MFA)
- Webhooks and integrations

## What Clerk-rs Provides

Clerk-rs serves as a bridge between Rust applications and Clerk's REST API, offering:

### 1. **Complete API Coverage**
The SDK provides full coverage of Clerk's Backend API, including:
- **User Management**: Create, read, update, delete users; manage user metadata and profiles
- **Authentication**: Session verification, token management, password verification
- **Organizations**: Create and manage organizations, memberships, and invitations
- **Sessions**: Session creation, verification, and revocation
- **Communication**: Email and SMS messaging capabilities
- **Templates**: JWT template management for custom tokens
- **Webhooks**: Integration with Svix for webhook management
- **Security Features**: Allow/block lists, MFA management, JWKS support

### 2. **Framework Integration**
Clerk-rs includes built-in middleware and validators for popular Rust web frameworks:

#### **Actix Web**
```rust
use clerk_rs::validators::actix::ClerkMiddleware;

App::new()
    .wrap(ClerkMiddleware::new(jwks_provider, None, true))
    .route("/protected", web::get().to(handler))
```

#### **Axum**
```rust
use clerk_rs::validators::axum::ClerkLayer;

let app = Router::new()
    .route("/protected", get(handler))
    .layer(ClerkLayer::new(jwks_provider, None, true));
```

#### **Rocket**
```rust
use clerk_rs::validators::rocket::ClerkGuard;

#[get("/")]
fn protected(jwt: ClerkGuard<MemoryCacheJwksProvider>) -> &'static str {
    "Authenticated!"
}
```

#### **Poem**
```rust
use clerk_rs::validators::poem::ClerkPoemMiddleware;

let app = Route::new()
    .at("/protected", get(handler))
    .with(ClerkPoemMiddleware::new(jwks_provider, true, None));
```

### 3. **Authentication & Authorization**
- **JWT Verification**: Built-in JWT token validation using JWKS (JSON Web Key Set)
- **Session Cookie Support**: Validate `__session` cookies for same-origin requests
- **Memory Caching**: Efficient JWKS caching with `MemoryCacheJwksProvider`
- **Configurable Security**: Customizable authentication rules and exclusions

### 4. **HTTP Client Flexibility**
- Built on `reqwest` for reliable HTTP communication
- Support for both `rustls-tls` (default) and `native-tls`
- Async/await throughout with `tokio` support
- Proper error handling with structured error types

## Architecture

### Core Components

1. **Clerk Client**: Main entry point for API interactions
2. **Configuration**: Centralized configuration management with API keys and endpoints
3. **APIs**: Organized modules for different Clerk service areas (users, organizations, sessions, etc.)
4. **Models**: Strongly-typed Rust structs for all Clerk API objects
5. **Validators**: Framework-specific middleware for request authentication
6. **Endpoints**: Enumerated API endpoints for type-safe requests

### Key Features

- **Type Safety**: Full Rust type system integration with compile-time guarantees
- **Async First**: Built for modern async Rust applications
- **Modular Design**: Use only the components you need via feature flags
- **Production Ready**: Used by companies like Tembo, Rezon, Gitar, and others
- **Community Maintained**: Active development with frequent updates aligned to Clerk's API changes

## Usage Examples

### Basic API Usage
```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
    let client = Clerk::new(config);
    
    let users = client.get(ClerkGetEndpoint::GetUserList).await?;
    Ok(())
}
```

### Email Creation
```rust
use clerk_rs::{apis::emails_api::Email, models::CreateEmailRequest};

let new_email = CreateEmailRequest::new();
Email::create(&client, Some(new_email)).await?;
```

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
clerk-rs = "0.4.1"

# Optional framework features
clerk-rs = { version = "0.4.1", features = ["actix"] }
clerk-rs = { version = "0.4.1", features = ["axum"] }
clerk-rs = { version = "0.4.1", features = ["rocket"] }
clerk-rs = { version = "0.4.1", features = ["poem"] }
```

## Why Choose Clerk-rs?

1. **Complete Solution**: Everything needed for user management in Rust applications
2. **Framework Agnostic**: Works with popular Rust web frameworks out of the box
3. **Type Safe**: Leverages Rust's type system for compile-time correctness
4. **Production Proven**: Trusted by multiple production applications
5. **Active Maintenance**: Regular updates to stay current with Clerk's API
6. **Open Source**: MIT licensed with community contributions welcomed

## Perfect For

- **SaaS Applications**: Multi-tenant applications with organization support
- **API Services**: Backend services requiring robust authentication
- **Web Applications**: Full-stack Rust web apps with user management needs
- **Microservices**: Service-to-service authentication in Rust ecosystems

Clerk-rs eliminates the complexity of building authentication from scratch, allowing Rust developers to focus on their core application logic while providing enterprise-grade user management capabilities.