# Clerk-rs: Official Community Rust SDK for Clerk

## Overview

**Clerk-rs** is the official community-maintained Rust SDK for [Clerk.dev](https://clerk.com), a complete authentication and user management solution. This SDK provides Rust developers with a comprehensive toolkit to integrate Clerk's authentication services into their applications.

## What is Clerk-rs?

Clerk-rs is a Rust library that offers:

- **Full Clerk API Integration**: Complete access to Clerk's Backend API with type-safe Rust bindings
- **JWT Validation**: Built-in JWT token validation with JWKS support  
- **Web Framework Middleware**: Ready-to-use authentication middleware for popular Rust web frameworks
- **Session Management**: Handle user sessions and authentication cookies
- **Organization Support**: Full support for Clerk's organization features with role-based permissions

## Key Features

### 🔐 Authentication & Authorization
- JWT token validation with RS256 algorithm support
- Session cookie validation (`__session` cookie support)
- Role-based access control (RBAC) with organization support
- Permission checking utilities

### 🌐 Web Framework Integration
Clerk-rs provides authentication middleware for popular Rust web frameworks:

- **Actix Web**: `ClerkMiddleware` for protecting routes
- **Axum**: `ClerkLayer` for authentication layers  
- **Rocket**: `ClerkGuard` for request guards
- **Poem**: `ClerkPoemMiddleware` for endpoint protection

### 📡 Complete API Coverage
The SDK provides access to all Clerk API endpoints including:

- User management (create, update, delete users)
- Email and phone number verification
- Organization management and invitations
- JWT template management
- Webhook handling
- Session management
- And much more...

## How It Works

### Basic API Usage
```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
let client = Clerk::new(config);

let users = client.get(ClerkGetEndpoint::GetUserList).await?;
```

### Middleware Protection
```rust
// Actix Web example
use clerk_rs::validators::{actix::ClerkMiddleware, jwks::MemoryCacheJwksProvider};

App::new()
    .wrap(ClerkMiddleware::new(MemoryCacheJwksProvider::new(clerk), None, true))
    .route("/protected", web::get().to(protected_handler))
```

## Architecture

### Core Components

1. **Clerk Client**: Main client for making API requests to Clerk's backend
2. **Configuration**: Handles API keys, base URLs, and client configuration
3. **Models**: Type-safe Rust structs for all Clerk API entities
4. **Validators**: JWT validation and authentication logic
5. **Framework Integrations**: Middleware implementations for web frameworks

### JWT Validation Flow

1. Extract JWT from Authorization header or `__session` cookie
2. Parse JWT header to get the Key ID (kid)
3. Fetch corresponding public key from Clerk's JWKS endpoint  
4. Validate JWT signature and claims (expiration, issuer, etc.)
5. Return parsed claims with user and organization information

## Production Usage

Clerk-rs is actively used in production by several companies:

- **[Tembo](https://tembo.io)** - Postgres cloud platform
- **[Rezon](https://rezon.ai)** - AI-powered applications  
- **[Gitar](https://gitar.co)** - Development tools
- **[Have I Been Squatted](https://haveibeensquatted.com)** - Security tools

## Technical Details

- **Language**: Rust (2021 edition)
- **HTTP Client**: Built on `reqwest` with async/await support
- **JWT Handling**: Uses `jsonwebtoken` crate for token validation
- **Serialization**: `serde` for JSON handling with Clerk API
- **Async Runtime**: Compatible with Tokio and other async runtimes

## Why Use Clerk-rs?

### For Rust Developers
- **Type Safety**: Full type safety with Rust's ownership system
- **Performance**: Zero-cost abstractions and efficient async operations  
- **Framework Agnostic**: Works with any Rust web framework
- **Production Ready**: Used by real companies in production environments

### For Authentication
- **Complete Solution**: Handles all aspects of authentication and user management
- **Standards Compliant**: Implements OAuth, JWT, and other web standards
- **Scalable**: Built to handle applications from startup to enterprise scale
- **Developer Experience**: Clean APIs and comprehensive documentation

## Conclusion

Clerk-rs represents a mature, production-ready authentication solution for the Rust ecosystem. It bridges the gap between Clerk's powerful authentication platform and Rust's performance-focused web development, providing developers with the tools they need to implement secure, scalable authentication in their applications.

Whether you're building a simple web API or a complex multi-tenant application, Clerk-rs provides the authentication primitives you need while maintaining Rust's emphasis on safety and performance.

---

*This summary was created by analyzing the [clerk-rs repository](https://github.com/DarrenBaldwin07/clerk-rs) and its comprehensive documentation.*