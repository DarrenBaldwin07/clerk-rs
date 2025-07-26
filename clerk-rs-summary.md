# Clerk-rs: The Official Community Rust SDK for Clerk

## Overview

Clerk-rs is the official community-maintained Rust SDK for the [Clerk](https://clerk.com) authentication and user management platform. This comprehensive SDK provides Rust developers with a complete toolkit for integrating Clerk's authentication services into their applications, supporting multiple popular web frameworks and offering both high-level API methods and low-level HTTP client functionality.

## What is Clerk?

Clerk is a modern authentication and user management service that provides features like user sign-up/sign-in, session management, multi-factor authentication, organization management, and more. It's designed to handle the complexity of authentication so developers can focus on building their core application features.

## Key Features

### 🚀 **Complete API Coverage**
- Full implementation of the Clerk Backend API
- Support for all major Clerk endpoints including users, sessions, organizations, invitations, and more
- Auto-generated models and API methods based on the official OpenAPI specification

### 🌐 **Multi-Framework Support**
The SDK provides native middleware and authentication guards for popular Rust web frameworks:
- **Actix Web**: Complete middleware integration with request guards
- **Axum**: Layer-based middleware with extension extractors
- **Rocket**: Guard-based authentication with dependency injection
- **Poem**: Middleware support for route protection

### 🔐 **JWT Validation & Session Management**
- Built-in JWT token validation using JWKS (JSON Web Key Set)
- Memory-cached JWKS provider for performance
- Session cookie validation for same-origin requests
- Automatic token verification and user extraction

### 📝 **Type-Safe API Client**
- Strongly typed request/response models
- Async/await support with Tokio runtime
- Built on reqwest for reliable HTTP communication
- Comprehensive error handling

## Architecture & Structure

### Core Components

1. **Clerk Client** (`src/clerk.rs`): The main SDK client providing HTTP methods (GET, POST, PUT, DELETE, PATCH) for interacting with Clerk APIs
2. **API Modules** (`src/apis/`): Organized API endpoints for different Clerk services
3. **Models** (`src/models/`): Type-safe Rust structs representing Clerk data structures
4. **Validators** (`src/validators/`): Framework-specific middleware and authentication guards
5. **Configuration** (`src/apis/configuration.rs`): SDK configuration management

### Framework Integration

Each supported web framework gets its own validator module:
- **Actix Web**: `ClerkMiddleware` for request interception
- **Axum**: `ClerkLayer` for tower-based middleware
- **Rocket**: `ClerkGuard` for request guards
- **Poem**: `ClerkPoemMiddleware` for poem-specific integration

## Usage Examples

### Basic API Usage
```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration, endpoints::ClerkGetEndpoint};

let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
let client = Clerk::new(config);
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

let app = Router::new()
    .route("/protected", get(protected_handler))
    .layer(ClerkLayer::new(MemoryCacheJwksProvider::new(clerk), None, true));
```

## Production Adoption

The SDK is actively used by several production applications including:
- **Tembo**: Database-as-a-Service platform
- **Rezon**: AI-powered application
- **Gitar**: Developer tools platform
- **Have I Been Squatted**: Security monitoring service

## Technical Implementation

### Dependencies
- **Core**: `serde`, `reqwest`, `jsonwebtoken`
- **Framework Support**: Optional dependencies for `actix-web`, `axum`, `rocket`, `poem`
- **Async Runtime**: Built for Tokio async runtime
- **HTTP Client**: Uses reqwest with configurable TLS (native-tls or rustls)

### Features
- Modular feature flags for framework-specific functionality
- Comprehensive documentation with examples
- Regular updates to match Clerk API changes
- MIT licensed for commercial and open-source use

## Why Clerk-rs?

1. **Community-Driven**: Actively maintained by the Rust community with frequent updates
2. **Framework Agnostic**: Works with all major Rust web frameworks
3. **Production Ready**: Used by multiple production applications
4. **Type Safety**: Leverages Rust's type system for compile-time safety
5. **Performance**: Efficient JWT validation with caching
6. **Comprehensive**: Covers the entire Clerk API surface area

## Getting Started

Add to your `Cargo.toml`:
```toml
[dependencies]
clerk-rs = "0.4"

# Enable framework-specific features
clerk-rs = { version = "0.4", features = ["axum"] }
```

Clerk-rs represents a mature, production-ready solution for Rust developers looking to integrate modern authentication and user management into their applications while maintaining the performance and safety guarantees that Rust developers expect.