# Clerk-rs: Official Community Rust SDK for Clerk

## Overview

**clerk-rs** is the official community-maintained Rust SDK for integrating with [Clerk](https://clerk.com), a modern authentication and user management platform. This crate provides a comprehensive, type-safe interface to the Clerk Backend API, enabling Rust developers to easily implement authentication, user management, and session handling in their applications.

## Key Features

### 🔧 **Complete API Coverage**
- Full implementation of the Clerk Backend API
- Generated from official OpenAPI specifications
- Support for all Clerk resources: users, sessions, organizations, invitations, JWT templates, and more

### 🚀 **Framework Integration**
The SDK provides first-class middleware support for popular Rust web frameworks:
- **Actix-web** - Complete middleware with session validation
- **Axum** - Layer-based authentication middleware  
- **Rocket** - Guard-based request protection
- **Poem** - Middleware for Poem v3

### 🔐 **Authentication & Authorization**
- JWT token validation with JWKS (JSON Web Key Set) support
- Session cookie validation for same-origin requests
- Memory-cached JWKS provider for optimal performance
- Automatic token verification and user context extraction

### 📡 **HTTP Client Features**
- Built on `reqwest` for reliable HTTP communication
- Support for both native-tls and rustls-tls
- Async/await throughout with tokio compatibility
- Comprehensive error handling

## Core Components

### Main Client (`Clerk`)
The central client provides methods for all HTTP operations:
- `get()`, `post()`, `put()`, `delete()`, `patch()` - Basic CRUD operations
- Parameter-based variants for dynamic endpoint construction
- Automatic request/response serialization with serde

### Models & APIs
- **80+ auto-generated models** covering all Clerk data structures
- **20+ API modules** organized by functionality (users, sessions, organizations, etc.)
- Type-safe request/response handling with comprehensive validation

### Middleware & Validators
- **Framework-agnostic core** with pluggable authentication providers
- **Memory-cached JWKS** provider for JWT validation
- **Cookie-based session** validation for web applications
- **Flexible configuration** for custom authentication flows

## Usage Examples

### Basic API Usage
```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
let client = Clerk::new(config);

// Get user list
let users = client.get(ClerkGetEndpoint::GetUserList).await?;
```

### Framework Integration (Axum)
```rust
use axum::{routing::get, Router};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

let config = ClerkConfiguration::new(None, None, Some("sk_key".to_string()), None);
let clerk = Clerk::new(config);

let app = Router::new()
    .route("/protected", get(handler))
    .layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), None, true));
```

## Production Usage

The SDK is actively used in production by several companies:
- [Tembo](https://tembo.io) - Database platform
- [Rezon](https://rezon.ai) - AI platform  
- [Gitar](https://gitar.co) - Development tools
- [Have I Been Squatted](https://haveibeensquatted.com) - Security service

## Technical Architecture

### Generated Code Base
The majority of the codebase is auto-generated from Clerk's OpenAPI specification, ensuring:
- **API Consistency** - Always up-to-date with Clerk's backend
- **Type Safety** - Compile-time validation of requests/responses
- **Comprehensive Coverage** - All endpoints and models included

### Modular Design
- `src/apis/` - API endpoint implementations
- `src/models/` - Data structures and types
- `src/validators/` - Framework-specific middleware
- `examples/` - Integration examples for each supported framework

### Dependencies
Key dependencies include:
- `reqwest` - HTTP client
- `serde` - Serialization/deserialization
- `jsonwebtoken` - JWT validation
- `async-trait` - Async trait support
- Framework-specific crates (optional features)

## Development Status

- **Version**: 0.4.1
- **License**: MIT
- **Maintenance**: Actively maintained by [DarrenBaldwin07](https://github.com/DarrenBaldwin07)
- **Repository**: [GitHub](https://github.com/DarrenBaldwin07/clerk-rs)
- **Documentation**: [docs.rs](https://docs.rs/clerk-rs)

The SDK receives frequent updates to maintain compatibility with Clerk's evolving API and incorporates community feedback for improved developer experience.

## Why Clerk-rs?

For Rust developers building applications that require authentication, clerk-rs provides:

1. **Official Support** - Community-maintained but officially recognized
2. **Type Safety** - Leverages Rust's type system for compile-time correctness
3. **Performance** - Async throughout with efficient JWKS caching
4. **Flexibility** - Works with multiple frameworks and custom implementations
5. **Production Ready** - Used by real companies in production environments

This makes clerk-rs an excellent choice for Rust applications requiring robust, scalable authentication and user management capabilities.