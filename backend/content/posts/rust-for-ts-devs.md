# Rust for TypeScript Developers

Welcome to Rust and Axum! Here are key mappings to keep in mind:

- **No `null` or `undefined`**: Rust uses `Option<T>` (`Some` or `None`).
- **No exceptions**: Rust uses `Result<T, E>` (`Ok` or `Err`).
- **Extractors instead of req.body/params**: Axum inspects function parameters like `Path(slug)` or `Json(body)`.
- **Dependency Injection**: Axum provides `State(state)` which is thread-safe and verified at compile time.

Enjoy writing high-performance APIs!

