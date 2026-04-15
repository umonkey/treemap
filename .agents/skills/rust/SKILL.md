---
name: rust
description: Coding style and formatting rules for Rust code. Use this whenever modifying any Rust files.
---

## Formatting rules

- Multi-line statements (blocks or those using brackets) must have one empty line between them and other code on the same indentation level, above and beyond.

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

## Development Workflow

- Formatting: always run `make format` after changing code.
- Verification: always run `make check` to run clippy and ensure the code compiles without warnings.
- Tests: run `make test` for unit tests.
