# Developement hints

## Running the application

We use Docker Compose to run the development version.  Start it with `docker compose up` or `make start` in the root of the repository.


## Rust Development

The backend is written in Rust.  Rust binaries are slow to compile.  It helps to use [sccache](https://crates.io/crates/sccache), especially on build servers like CI/CD.
