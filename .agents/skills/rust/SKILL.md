---
name: rust
description: Rust style, structural simplification, DI pattern, and error boundary standards for services/backend/ and services/chatbot/. Use when modifying any *.rs files or Cargo.toml.
---

# Skill: rust

## Formatting rules

- Multi-line statements (blocks or those using brackets) must have one empty line between them and other code on the same indentation level, both above and below.

## Structural Simplification

- Re-read your own diff before finishing. When the same short sequence appears once per type, per field, or per item, extract a private method and iterate over it. Do this without waiting for the operator to point it out.
- Register parallel items in one place only when they already share a call shape. If adding an item means editing several duplicated blocks (name, fetch, log, push), replace them with a single array or iterator so the next addition is one entry, but do not invent a wrapper type just to make unrelated items look alike.
- Match the abstraction to the data. A list of names is enough when one call accepts the name, such as a storage driver listing by bucket. When each item is a distinct wrapper with no shared call, a name list still needs dispatch, so do not add an enum or trait just to remove the repetition.
- Keep per-item work sequential when items are expensive to hold. Do not collect every item into a Vec up front only to loop over it later: list one, process one.
- Apply the same rule to adjacent dead weight: remove a stale `#[allow(dead_code)]` in the change that makes the item used, because an unnecessary allowance never fails the gate.
- Preserve behavior while restructuring. Output order, log text, and error propagation must stay identical unless the task is explicitly about them.
- Do not over-abstract. Two or three lines that read clearly are fine. Extract only when the duplication is a real editing hazard or a third copy appears, and only if the extraction is shorter than the duplication it removes. A new type whose only purpose is to hold heterogeneous items is a net loss.

## Dependency Injection Pattern

The project uses an on-the-fly dependency injection system based on `Context` and `Injectable` traits.

### Core Traits

- `Context`: a trait implemented by `AppState` that provides access to core infrastructure (Database, Config, Queue, etc.).
- `Injectable`: a trait that services and repositories implement to define how they are constructed from a `Context`.
- `ContextExt`: provides the `.build::<T>()` method on any context to instantiate an `Injectable` type.

### Creating a New Service or Repository

1. Define your struct.
2. Implement `Injectable` for it.
3. Use `ctx.build::<OtherType>()?` to resolve dependencies that also implement `Injectable`.
4. Use `ctx.some_client()` or similar for core infrastructure dependencies provided by `Context`.

Example:

```rust
impl Injectable for MyService {
    fn inject(ctx: &dyn Context) -> Result<Self> {
        Ok(Self {
            db: ctx.database(),
            repo: Arc::new(ctx.build::<MyRepository>()?),
            osm: Arc::new(ctx.build::<OsmClient>()?),
        })
    }
}
```

### Using Services in Actions

In Actix-Web handlers, use the `build` method on the `Data<AppState>`:

```rust
pub async fn my_action(state: Data<AppState>) -> Result<Json<...>> {
    let service = state.build::<MyService>()?;
    let result = service.do_something().await?;
    Ok(Json(result))
}
```

## Error Handling

- Don't log where you throw: avoid logging errors (e.g., `error!` or `warn!`) in services, repositories, or logic that simply detects or propagates the error.
- Log at the boundary: errors should be logged once at the application boundary (e.g., in Actix-Web's `ResponseError` implementation).
- Provide context: when creating errors, include enough information (e.g., field names, IDs) so the person reading the log can identify the cause without needing a stack trace.

## Database Queries (libSQL)

- Parameterless queries: when executing a query with no parameters on a prepared statement with libSQL, use `params_from_iter(std::iter::empty::<Value>())` to avoid `IntoParams` type inference ambiguities:

```rust
let mut rows = stmt.query(params_from_iter(std::iter::empty::<Value>())).await?;
```

## Development Workflow

- After implementing the required changes, verify code integrity by running `make format check` in the service directory.
- Do not run the full gate twice in one service at the same time. It shares one `target/` directory and the backend suite shares `var/test-*.db`, so parallel runs block on the Cargo lock or interfere through the test database. When several scopes work in one service, give each `cargo clippy -- -D warnings`, `cargo check --tests`, and `cargo fmt --check`, then run `make check` once, serially, at the end.
