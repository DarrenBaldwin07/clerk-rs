# Clerk-rs: Official Community Rust SDK for Clerk Authentication

[![crates.io](https://img.shields.io/crates/v/clerk-rs?style=flat-square)](https://crates.io/crates/clerk-rs)
[![Downloads](https://img.shields.io/crates/d/clerk-rs.svg?style=flat-square)](https://crates.io/crates/clerk-rs)
[![docs.rs](https://img.shields.io/docsrs/clerk-rs?style=flat-square)](https://docs.rs/clerk-rs)

## Overview

**clerk-rs** is the official community-maintained Rust SDK for [Clerk](https://clerk.com), a complete authentication and user management platform. This crate provides a comprehensive Rust interface to the Clerk Backend API, enabling developers to integrate authentication, user management, and organization features into their Rust applications seamlessly.

## Key Features

### 🔐 Complete Authentication Support
- User authentication and session management
- JWT token validation and verification
- Support for multiple authentication methods (email, phone, OAuth)
- Multi-factor authentication (MFA) capabilities

### 🏢 Organization Management
- Create and manage organizations
- Organization memberships and invitations
- Role-based access control
- Organization metadata management

### 🛡️ Web Framework Integration
The SDK provides middleware and guards for popular Rust web frameworks:
- **Actix Web**: `ClerkMiddleware` for request authentication
- **Axum**: `ClerkLayer` for layer-based authentication
- **Rocket**: `ClerkGuard` for request guards
- **Poem**: `ClerkPoemMiddleware` for poem framework

### 📧 Communication Features
- Email and SMS message creation
- Template management for emails and SMS
- Webhook integration with Svix

### 🔧 Developer Experience
- Async/await support with tokio
- Comprehensive error handling
- Type-safe API with serde serialization
- In-memory JWKS caching for performance
- Extensive documentation and examples

## Architecture

The SDK is structured into several key modules:

- **`apis/`**: API endpoint implementations for all Clerk services
- **`models/`**: Type definitions for all Clerk data structures
- **`validators/`**: Framework-specific middleware and authentication validators
- **`clerk/`**: Core client implementation
- **`endpoints/`**: Endpoint definitions and routing

## Usage Examples

### Basic Client Setup
```rust
use clerk_rs::{clerk::Clerk, ClerkConfiguration};

let config = ClerkConfiguration::new(None, None, Some("sk_test_key".to_string()), None);
let client = Clerk::new(config);
```

### Protecting Axum Routes
```rust
use axum::{routing::get, Router};
use clerk_rs::{
    clerk::Clerk,
    validators::{axum::ClerkLayer, jwks::MemoryCacheJwksProvider},
    ClerkConfiguration,
};

let app = Router::new()
    .route("/protected", get(protected_handler))
    .layer(ClerkLayer::new(
        MemoryCacheJwksProvider::new(clerk), 
        Some(vec!["/protected".to_string()]), 
        true
    ));
```

### User Management
```rust
use clerk_rs::apis::users_api::User;

// Create a new user
let user = User::create(&client, create_user_request).await?;

// Get user list
let users = client.get(ClerkGetEndpoint::GetUserList).await?;
```

## Production Ready

The SDK is actively used in production by several companies including:
- [Tembo](https://tembo.io) - Postgres platform
- [Rezon](https://rezon.ai) - AI platform
- [Gitar](https://gitar.co) - Development analytics
- [Have I Been Squatted](https://haveibeensquatted.com) - Security service

## Technical Specifications

- **Language**: Rust (2021 edition)
- **HTTP Client**: reqwest with JSON and multipart support
- **Async Runtime**: Compatible with tokio
- **Serialization**: serde with derive macros
- **JWT Handling**: jsonwebtoken crate
- **TLS Support**: Both rustls-tls (default) and native-tls options

## API Coverage

The SDK provides complete coverage of the Clerk Backend API including:

- **User Management**: CRUD operations, metadata, authentication
- **Session Management**: Token creation, verification, revocation
- **Organization Management**: Organizations, memberships, invitations
- **Security Features**: Allow/block lists, MFA, JWT templates
- **Communication**: Email/SMS templates and messaging
- **Webhooks**: Svix integration for event handling
- **Instance Management**: Configuration and restrictions

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
clerk-rs = "0.4.1"

# Optional framework features
clerk-rs = { version = "0.4.1", features = ["axum"] }
clerk-rs = { version = "0.4.1", features = ["actix"] }
clerk-rs = { version = "0.4.1", features = ["rocket"] }
```

## Why Choose clerk-rs?

1. **Official Community Support**: Maintained with close collaboration with Clerk
2. **Type Safety**: Full Rust type system benefits with comprehensive error handling
3. **Performance**: Efficient async implementation with connection pooling
4. **Framework Agnostic**: Works with popular Rust web frameworks out of the box
5. **Production Tested**: Used by multiple companies in production environments
6. **Active Development**: Regularly updated to match Clerk API changes

The clerk-rs SDK represents a mature, production-ready solution for integrating Clerk's authentication platform into Rust applications, offering both ease of use and the performance benefits expected from Rust tooling.

---

*For detailed API documentation, visit the [official Clerk Backend API docs](https://clerk.com/docs/reference/backend-api) and the [clerk-rs documentation](https://docs.rs/clerk-rs).*