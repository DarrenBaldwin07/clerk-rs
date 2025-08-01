# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.4.1] - 2025-06-07

### Fixed
- Added missing `log` dependency to Cargo.toml to resolve compilation issues
- Fixed dependency resolution for logging functionality

### Changed
- Updated code formatting across the entire codebase

### Technical Details
- The `log` crate (version 0.4.27) is now properly declared as a dependency
- This resolves issues where logging functionality was used but the dependency wasn't explicitly declared
- All code has been formatted according to the project's rustfmt configuration

### Contributors
- [@shogo-nakano-desu](https://github.com/shogo-nakano-desu) - Fixed log dependency issue

---

## Previous Releases

### [0.4.0] and earlier
- Comprehensive Clerk API SDK implementation
- Support for multiple web frameworks (Actix, Axum, Rocket, Poem)
- JWT validation and session management
- Complete API coverage for Clerk's backend API
- Middleware support for popular Rust web frameworks
- Production-ready authentication and authorization features