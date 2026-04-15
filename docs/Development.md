# Developement hints

## Running the application

We use Docker Compose to run the development version. Start it with `docker compose up` or `make start` in the root of the repository.

## Rust Development

The backend is written in Rust. Rust binaries are slow to compile. It helps to use [sccache](https://crates.io/crates/sccache), especially on build servers like CI/CD.

## Debugging

### Backend Logs

The logs are written to stdout and then handled by Docker depending on how you set it up.

For local setups, you can use `docker logs` to read or tail them. When running in Fly.io, use the [logging console](https://fly.io/apps/treemap-blue-frog-3927/monitoring): it shows recent logs and has a link to a built in Grafana instance which can be used to query logs and search for something specific.
