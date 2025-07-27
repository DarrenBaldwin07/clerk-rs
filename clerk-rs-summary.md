# Clerk-rs: Official Community Rust SDK for Clerk

## Overview

**clerk-rs** is the official community-maintained Rust SDK for [Clerk](https://clerk.com), a modern authentication and user management platform. This comprehensive SDK provides Rust developers with a complete toolkit for integrating Clerk's authentication services into their applications.

## What is Clerk-rs?

Clerk-rs is a Rust library that wraps the Clerk Backend API, making it easy for Rust developers to:

- **Authenticate users** in Rust web applications
- **Manage user data** through Clerk's comprehensive API
- **Protect API endpoints** with JWT validation middleware
- **Handle organizations** and user memberships
- **Send emails and SMS** through Clerk's messaging system

## Key Features

### 🔐 **Authentication & Authorization**
- JWT token validation and verification
- Session management
- User authentication workflows
- Password verification and management

### 🌐 **Framework Integration**
The SDK provides first-class middleware support for popular Rust web frameworks:
- **Actix Web** - Full middleware support for protecting routes
- **Axum** - Layer-based authentication integration
- **Rocket** - Guard-based route protection
- **Poem** - Middleware for poem v3 framework

### 📧 **Communication**
- Email management and sending
- SMS messaging capabilities
- Template management for emails and SMS
- Custom email/SMS template creation

### 👥 **Organization Management**
- Create and manage organizations
- Handle organization memberships
- Organization invitations and permissions
- Member role management

### 🛠 **Developer Experience**
- **Type-safe API** - All API responses are strongly typed
- **Async/await support** - Built on tokio for async operations
- **Comprehensive documentation** - Detailed docs and examples
- **Production ready** - Used by companies like Tembo, Rezon, and Gitar

## Architecture

The SDK is structured into several key modules:

- **`apis/`** - Contains all API client implementations for different Clerk endpoints
- **`models/`** - Strongly-typed data structures for API requests/responses  
- **`validators/`** - Framework-specific middleware for JWT validation
- **`clerk.rs`** - Main client for making API calls
- **`endpoints.rs`** - Endpoint definitions and routing

## Usage Examples

### Basic API Usage
```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
let client = Clerk::new(config);
let users = client.get(ClerkGetEndpoint::GetUserList).await?;
```

### Protecting Routes with Actix Web
```rust
use actix_web::{web, App, HttpServer};
use clerk_rs::validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider};

HttpServer::new(|| {
    let config = ClerkConfiguration::new(None, None, Some("your_secret_key".to_string()), None);
    let clerk = Clerk::new(config);

    App::new()
        .wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
        .route("/protected", web::get().to(handler))
})
```

## API Coverage

The SDK provides comprehensive coverage of the Clerk Backend API, including:

- **User Management** - Create, update, delete users
- **Session Management** - Validate and manage user sessions  
- **Organization APIs** - Full organization and membership management
- **Email/SMS APIs** - Send messages and manage templates
- **JWT Templates** - Create custom JWT token templates
- **Webhooks** - Handle Clerk webhook events
- **Allowlist/Blocklist** - Manage user access controls

## Production Usage

Clerk-rs is actively used in production by several companies:
- **[Tembo](https://tembo.io)** - Postgres platform
- **[Rezon](https://rezon.ai)** - AI platform  
- **[Gitar](https://gitar.co)** - Development tools
- **[Have I Been Squatted](https://haveibeensquatted.com)** - Security service

## Why Choose Clerk-rs?

1. **Official Community Support** - Maintained with Clerk's backing
2. **Type Safety** - Leverages Rust's type system for API safety
3. **Framework Agnostic** - Works with all major Rust web frameworks
4. **Production Ready** - Battle-tested by real applications
5. **Active Development** - Frequently updated to match Clerk API changes
6. **Comprehensive** - Supports all major Clerk features

## Technical Details

- **Language**: Rust (Edition 2021)
- **Async Runtime**: Built on tokio
- **HTTP Client**: Uses reqwest for API calls
- **Serialization**: Powered by serde for JSON handling
- **JWT Handling**: Uses jsonwebtoken crate for validation
- **Version**: Currently at v0.4.1 on crates.io

## Getting Started

Add clerk-rs to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4.1"
```

Enable framework-specific features as needed:
- `actix` - For Actix Web integration
- `axum` - For Axum framework support  
- `rocket` - For Rocket framework support
- `poem` - For Poem framework support

## Links

- **📦 [Crates.io](https://crates.io/crates/clerk-rs)** - Official package registry
- **📚 [Docs.rs](https://docs.rs/clerk-rs)** - API documentation
- **🐙 [GitHub](https://github.com/DarrenBaldwin07/clerk-rs)** - Source code and issues
- **🌐 [Clerk Documentation](https://clerk.com/docs/reference/backend-api)** - Official Clerk API docs

---

*Clerk-rs makes it simple to add secure, scalable authentication to any Rust application. Whether you're building a web API, microservice, or full-stack application, clerk-rs provides the tools you need to implement modern authentication patterns with minimal boilerplate.*