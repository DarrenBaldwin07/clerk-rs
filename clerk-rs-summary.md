# Clerk-rs: Official Community Rust SDK for Clerk

## Overview

**clerk-rs** is the official community-maintained Rust SDK for the [Clerk](https://clerk.com) authentication and user management platform. This powerful library provides a complete Rust interface to Clerk's Backend API, enabling developers to integrate robust authentication, user management, and organization features into their Rust applications with ease.

## What is Clerk?

Clerk is a complete authentication and user management platform that provides:
- User authentication (email/password, social logins, passwordless)
- User management and profiles
- Multi-factor authentication (MFA)
- Organization management with roles and permissions
- Session management and JWT validation
- Webhooks and real-time updates

## Key Features of clerk-rs

### 🚀 **Complete API Coverage**
- **User Management**: Create, update, delete, and manage user accounts
- **Authentication**: Session validation, JWT verification, and token management  
- **Organizations**: Full organization management with memberships, invitations, and roles
- **Email & SMS**: Send emails and SMS messages through Clerk's infrastructure
- **Templates**: Manage email/SMS templates and JWT templates
- **Webhooks**: Integration with Clerk's webhook system via Svix
- **Phone Numbers & Email Addresses**: Comprehensive contact management

### 🔒 **Advanced Security Features**
- **JWT Validation**: Built-in JWT verification with JWKS (JSON Web Key Set) support
- **Session Cookie Validation**: Support for `__session` cookie authentication
- **Multi-factor Authentication**: Complete MFA workflow support
- **Organization Permissions**: Role-based access control (RBAC) with fine-grained permissions

### 🌐 **Web Framework Integration**
The SDK provides first-class middleware and validators for popular Rust web frameworks:

- **Axum**: Layer-based middleware with request extraction
- **Actix Web**: Middleware for route protection and JWT extraction
- **Rocket**: Guard-based authentication with request guards
- **Poem**: Endpoint middleware for authentication

### ⚡ **Performance & Caching**
- **Memory-cached JWKS**: Efficient JWT key caching for high-performance validation
- **Async/Await**: Full async support using Tokio runtime
- **Reqwest HTTP Client**: Fast, reliable HTTP client with connection pooling

## Architecture

### Core Components

```
clerk-rs/
├── src/
│   ├── clerk.rs              # Main SDK client
│   ├── apis/                 # Generated API endpoints
│   ├── models/              # Data models and types
│   ├── validators/          # JWT validation and middleware
│   └── endpoints.rs         # Endpoint definitions
```

### Main Client (`Clerk`)
The central `Clerk` struct provides methods for all HTTP operations:
- `get()`, `post()`, `put()`, `patch()`, `delete()` - Basic HTTP methods
- `get_with_params()`, `post_with_params()` - Parameterized requests
- Full async/await support with `Result<Value, reqwest::Error>` returns

### Validation System
The validation system (`validators/`) provides:
- **JWT Verification**: RS256 algorithm support with JWKS key rotation
- **Framework Middleware**: Ready-to-use middleware for major web frameworks
- **Permission Checking**: Built-in organization permission and role validation
- **Cookie Support**: Automatic `__session` cookie validation for same-origin requests

## Usage Examples

### Basic API Usage
```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

let config = ClerkConfiguration::new(None, None, Some("sk_test_...".to_string()), None);
let client = Clerk::new(config);

// Get user list
let users = client.get(ClerkGetEndpoint::GetUserList).await?;

// Create a user
let new_user = CreateUserRequest { /* ... */ };
let user = client.post(ClerkPostEndpoint::CreateUser, new_user).await?;
```

### Axum Integration
```rust
use axum::{routing::get, Router, Extension};
use clerk_rs::{
    clerk::Clerk,
    validators::{authorizer::ClerkJwt, axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

async fn protected_route(Extension(jwt): Extension<ClerkJwt>) -> String {
    format!("Hello, {}!", jwt.sub)
}

let clerk = Clerk::new(config);
let app = Router::new()
    .route("/protected", get(protected_route))
    .layer(ClerkLayer::new(
        MemoryCacheJwksProvider::new(clerk), 
        Some(vec!["/protected".to_string()]), 
        true // validate session cookies
    ));
```

### Organization Permissions
```rust
// Check user permissions in JWT claims
if let Some(org) = &jwt.org {
    if org.has_permission("posts:write") {
        // User can write posts
    }
    
    if org.has_role("admin") {
        // User is an admin (use permissions instead when possible)
    }
}
```

## Production Usage

clerk-rs is actively used in production by several companies:
- **[Tembo](https://tembo.io)** - Postgres cloud platform
- **[Rezon](https://rezon.ai)** - AI-powered analytics
- **[Gitar](https://gitar.co)** - Development workflow tools
- **[Have I Been Squatted](https://haveibeensquatted.com)** - Domain security service

## Technical Details

### Dependencies
- **Serde**: JSON serialization/deserialization
- **Reqwest**: HTTP client with TLS support
- **jsonwebtoken**: JWT validation and parsing
- **async-trait**: Async trait support
- **Framework-specific**: Optional dependencies for Axum, Actix, Rocket, Poem

### Features
- `default`: Includes `rustls-tls` for secure connections
- `native-tls`: Use system's native TLS implementation
- `rustls-tls`: Use Rust's pure TLS implementation
- Framework features: `actix`, `axum`, `rocket`, `poem`

### Authentication Methods
1. **Bearer Token**: Standard `Authorization: Bearer <token>` header
2. **Session Cookie**: Automatic `__session` cookie validation for same-origin requests
3. **API Key**: Support for API key authentication (planned)

## Development & Contribution

The project is:
- ✅ **Open Source**: MIT licensed with active community contributions
- 🔄 **Actively Maintained**: Regular updates to match Clerk API changes
- 📖 **Well Documented**: Comprehensive docs and examples
- 🧪 **Well Tested**: Extensive test suite with mocked services
- 📦 **Published**: Available on [crates.io](https://crates.io/crates/clerk-rs)

## Getting Started

Add to your `Cargo.toml`:
```toml
[dependencies]
clerk-rs = "0.4.1"
# Add framework-specific features as needed
# clerk-rs = { version = "0.4.1", features = ["axum"] }
```

## Links

- 📚 **Documentation**: [docs.rs/clerk-rs](https://docs.rs/clerk-rs)
- 💻 **Repository**: [github.com/DarrenBaldwin07/clerk-rs](https://github.com/DarrenBaldwin07/clerk-rs)
- 📦 **Crate**: [crates.io/crates/clerk-rs](https://crates.io/crates/clerk-rs)
- 🌐 **Clerk Docs**: [clerk.com/docs/reference/backend-api](https://clerk.com/docs/reference/backend-api)

---

clerk-rs represents a mature, production-ready solution for integrating Clerk's powerful authentication and user management features into Rust applications. Whether you're building a simple web API or a complex multi-tenant SaaS application, clerk-rs provides the tools and abstractions needed to implement secure, scalable authentication with minimal boilerplate code.