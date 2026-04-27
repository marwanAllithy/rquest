# AGENTS.md

## Project
Single Rust package (edition 2024) implementing a TUI HTTP client. No workspaces or custom build steps.

## Dependencies Quirks
- `reqwest` uses the `blocking` feature: all HTTP calls must use synchronous APIs, async is not available.

## Dev Commands
Standard Cargo workflow: `cargo build`, `cargo run`, `cargo test`. No project-specific scripts or task runners.

## Structure
- Entrypoint: `src/main.rs`
- TUI tabs: `src/tabs/` (auth, body, headers, params, result, help, binds)
- Core logic: `src/app.rs`, `src/render.rs`
