# Clerk-rs: The Official Community Rust SDK for Clerk

## Overview

**Clerk-rs** is the official community-maintained Rust SDK for [Clerk](https://clerk.com), a comprehensive authentication and user management platform. This library provides Rust developers with a powerful, type-safe interface to integrate Clerk's authentication services into their applications.

## What is Clerk-rs?

Clerk-rs is a robust SDK that bridges the gap between Rust applications and Clerk's authentication API. It offers both direct API access and framework-specific middleware for popular Rust web frameworks, making it easy to add enterprise-grade authentication to Rust web applications.

## Key Features

### 🔐 **Complete API Coverage**
- Full implementation of the Clerk Backend API
- Support for all Clerk resources: users, sessions, organizations, invitations, and more
- Type-safe request/response models generated from Clerk's OpenAPI specification

### 🚀 **Framework Integration**
The SDK provides ready-to-use middleware and authentication guards for popular Rust web frameworks:
- **Axum** - Layer-based middleware with JWT extraction
- **Actix Web** - Middleware for route protection
- **Rocket** - Request guard system for authentication
- **Poem** - Built-in middleware support

### 🔒 **JWT Validation**
- Automatic JWT token validation using Clerk's JWKS (JSON Web Key Set)
- Session cookie support for same-origin requests
- Memory-cached JWKS provider for optimal performance
- Support for organization-based permissions and roles

### 📦 **Easy Configuration**
- Simple configuration with secret keys
- Environment-based setup options
- Flexible client configuration for different deployment scenarios

## Architecture

### Core Components

1. **Clerk Client (`Clerk`)**: Main interface for making API requests
2. **Configuration (`ClerkConfiguration`)**: Handles authentication and API settings
3. **Validators**: Framework-specific middleware for JWT validation
4. **Models**: Type-safe representations of Clerk API resources
5. **APIs**: Organized endpoint handlers for different Clerk resources

### Supported Operations

The SDK provides access to all major Clerk API functionality:

- **User Management**: Create, update, delete, and query users
- **Session Management**: Handle user sessions and tokens
- **Organization Management**: Multi-tenant organization support
- **Authentication**: JWT validation and session verification
- **Invitations**: User invitation and onboarding flows
- **Email/SMS**: Template management and messaging
- **Webhooks**: Integration with Clerk's webhook system

## Usage Examples

### Basic API Usage
```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
let client = Clerk::new(config);
let users = client.get(ClerkGetEndpoint::GetUserList).await?;
```

### Axum Integration
```rust
use clerk_rs::validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider};

let app = Router::new()
    .route("/protected", get(protected_handler))
    .layer(ClerkLayer::new(
        MemoryCacheJwksProvider::new(clerk), 
        Some(vec!["/protected".to_string()]), 
        true
    ));
```

### Authentication Guard
```rust
// In your route handler
async fn protected_route(Extension(clerk_jwt): Extension<ClerkJwt>) -> String {
    format!("Hello, {}!", clerk_jwt.sub)
}
```

## Target Audiences

### 🛠️ **Rust Web Developers**
Perfect for developers building web applications in Rust who need robust authentication without the complexity of rolling their own auth system.

### 🏢 **Enterprise Applications**
Ideal for companies building multi-tenant applications with organization-based access control and complex permission systems.

### 🚀 **SaaS Builders**
Excellent choice for SaaS applications that require user management, subscription handling, and secure API access.

## Production Usage

Clerk-rs is trusted by several production applications:
- **[Tembo](https://tembo.io)** - Database-as-a-Service platform
- **[Rezon](https://rezon.ai)** - AI-powered applications
- **[Gitar](https://gitar.co)** - Git analytics platform
- **[Have I Been Squatted](https://haveibeensquatted.com)** - Domain security service

## Technical Specifications

- **Language**: Rust (2021 edition)
- **HTTP Client**: Built on reqwest with async/await support
- **JWT Processing**: Uses jsonwebtoken for secure token validation
- **Serialization**: Leverages serde for type-safe JSON handling
- **Async Runtime**: Compatible with tokio and other async runtimes

## Installation

Add to your `Cargo.toml`:
```toml
[dependencies]
clerk-rs = "0.4.1"

# Enable framework features as needed
clerk-rs = { version = "0.4.1", features = ["axum"] }
```

## Why Choose Clerk-rs?

1. **Type Safety**: Full Rust type system benefits with compile-time guarantees
2. **Performance**: Efficient JWT caching and async processing
3. **Maintainability**: Regular updates to match Clerk API changes
4. **Community**: Active community support and contributions
5. **Documentation**: Comprehensive examples and API documentation
6. **Framework Agnostic**: Works with any Rust web framework

## Repository Information

- **GitHub**: [DarrenBaldwin07/clerk-rs](https://github.com/DarrenBaldwin07/clerk-rs)
- **Crates.io**: [clerk-rs](https://crates.io/crates/clerk-rs)
- **License**: MIT
- **Maintainer**: DarrenBaldwin07

## Conclusion

Clerk-rs represents a mature, production-ready solution for adding authentication to Rust web applications. Its combination of comprehensive API coverage, framework integration, and type safety makes it an excellent choice for developers who want to focus on building their application logic rather than authentication infrastructure.

Whether you're building a simple web service or a complex multi-tenant SaaS platform, clerk-rs provides the tools and abstractions necessary to implement secure, scalable authentication with minimal boilerplate code.