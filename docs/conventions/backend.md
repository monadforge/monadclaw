# Backend Conventions (Rust)

## Project Structure

```
src/
├── core/         # Agent loop, orchestration
├── memory/       # Short-term and long-term memory
├── providers/    # LLM provider implementations
├── interfaces/
│   ├── discord/  # Discord bot
│   └── api/      # REST API server
└── config/       # Configuration loading
```

## Code Conventions

- Use `thiserror` for error types, `anyhow` for propagation in binaries
- Each module exposes a clean public API; internals are `pub(crate)` or private
- Async runtime: Tokio
- No `unwrap()` or `expect()` in library code; only in tests or with a comment
- Prefer `impl Trait` in function arguments, `Box<dyn Trait>` for stored objects

## Provider Trait

All LLM providers implement a common trait. Adding a provider means implementing the trait, not modifying core logic.

## Configuration

- Config loaded at startup from file + environment variables
- Environment variables take precedence over file values
- No runtime config mutation; restart required for config changes
