# Lumen Blocks - Development Commands

# Show available commands
default:
    @just --list

# ─────────────────────────────────────────────────────────────────────────────
# Development
# ─────────────────────────────────────────────────────────────────────────────

# Start docsite dev server with hot-reload
dev:
    dx serve -p docsite --platform web

# Start docsite dev server (alias)
dev-docsite:
    dx serve -p docsite --platform web

# Start docsite with verbose logging
dev-verbose:
    RUST_LOG=debug dx serve -p docsite --platform web

# ─────────────────────────────────────────────────────────────────────────────
# Building
# ─────────────────────────────────────────────────────────────────────────────

# Build the library
build:
    cargo build -p lumen-blocks

# Build the library in release mode
build-release:
    cargo build -p lumen-blocks --release

# Build the docsite for production
build-docsite:
    dx bundle -p docsite --platform web --features analytics --release
    cp docsite/assets/_redirects target/dx/docsite/release/web/public

# Build documentation site docs
build-docs:
    cd docsite/docs && cargo build

# ─────────────────────────────────────────────────────────────────────────────
# Testing & Quality
# ─────────────────────────────────────────────────────────────────────────────

# Run all tests
test:
    cargo test -p lumen-blocks

# Check all packages compile
check:
    cargo check --workspace

# Format all code
fmt:
    cargo fmt --all

# Check formatting without changing files
fmt-check:
    cargo fmt --all -- --check

# Run clippy lints
lint:
    cargo clippy --workspace -- -D warnings

# Pre-commit checks (format + build check)
pre-commit:
    cargo fmt --all
    cargo check --workspace

# ─────────────────────────────────────────────────────────────────────────────
# Utilities
# ─────────────────────────────────────────────────────────────────────────────

# Clean build artifacts
clean:
    cargo clean
    rm -rf target/dx

# Watch for changes and run checks
watch:
    cargo watch -x "check --workspace"

# Update dependencies
update:
    cargo update

# Show outdated dependencies
outdated:
    cargo outdated
