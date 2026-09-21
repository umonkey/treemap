---
name: docker-build-caching
description: Dockerfile and GitHub Actions cache setup for this repo's container image builds. Use when editing any Dockerfile or .github/workflows/deploy-*.yml.
---

# Docker Build Caching

How to set up Dockerfiles and GitHub workflows so image builds reuse cached dependencies instead of recompiling everything on each run.

## Why builds rebuild from scratch

- BuildKit cache mounts (`RUN --mount=type=cache,...`) are never exported by `cache-to`, for any backend. GitHub-hosted runners are ephemeral, so a cache mount is cold on every run.
- A cold cargo registry makes cargo re-fetch and re-extract crate sources, which invalidates the cached `target/` fingerprints and recompiles the whole dependency graph.
- The GitHub Actions cache (`type=gha`) has a 10 GB per-repository limit and evicts least-recently-used entries. Large Rust layers get evicted when several workflows share it.
- A `cache-from` entry pointing at a normal image tag (for example `ghcr.io/umonkey/treemap-chatbot:latest`) is dead unless the image was pushed with registry cache metadata.

## Dockerfile rules

- Do not use `--mount=type=cache` for the cargo registry or `target` in CI-built images. Let them live in normal layers so the layer cache can export them.
- Keep the two-step dependency build: copy `Cargo.toml` and `Cargo.lock`, build a dummy `src/main.rs`, then copy the real source and rebuild. This keeps the dependency layer stable when only local source changes.
- Do not mount `~/.npm` as a cache mount either; let `npm ci` write into the layer.
- Keep the dependency build as an intermediate layer; it is only exported when the workflow uses `mode=max`.

Example Rust builder stage:

```dockerfile
FROM docker.io/library/rust:1.97-bookworm AS builder
RUN apt-get update && apt-get install -y libssl-dev pkg-config

WORKDIR /app
COPY Cargo.toml Cargo.lock ./

# STEP 1: build only dependencies.
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# STEP 2: build the application.
COPY src src
RUN touch src/main.rs
RUN cargo build --release && cp target/release/chatbot /app/chatbot-bin
```

## GitHub workflow rules

- Use the registry cache backend for large Rust builds. It has no 10 GB limit and survives Actions-cache eviction.
- Always set `mode=max`; the default `mode=min` drops the intermediate dependency layer.
- Read from both the registry cache and `type=gha`, so PRs can fall back to the Actions cache.
- Write the registry cache only on non-PR events. Fork PRs cannot push to ghcr, so they write `type=gha` instead.
- Keep a distinct `scope` per image when using `type=gha`.
- Do not add a bare image tag to `cache-from`; it is not a cache source.
- Run `docker/setup-buildx-action` before the build; the default docker driver cannot export `type=gha`.

Example build step:

```yaml
- name: Build and Push Docker Image
  uses: docker/build-push-action@v7
  with:
    context: services/chatbot
    push: ${{ github.ref == 'refs/heads/master' }}
    tags: ghcr.io/umonkey/treemap-chatbot:latest
    cache-from: |
      type=registry,ref=ghcr.io/umonkey/treemap-chatbot:buildcache
      type=gha,scope=chatbot
    cache-to: ${{ github.event_name == 'pull_request' && 'type=gha,scope=chatbot,mode=max' || 'type=registry,ref=ghcr.io/umonkey/treemap-chatbot:buildcache,mode=max' }}
```

Small images without cache mounts (for example caddy) can keep the simpler form:

```yaml
cache-from: type=gha,scope=caddy
cache-to: type=gha,mode=max,scope=caddy
```

## Checklist

- No `--mount=type=cache` in any Dockerfile built by CI.
- `cache-to` uses `mode=max`.
- The registry cache ref is `<image>:buildcache` and matches between `cache-from` and `cache-to`.
- `cache-to` is guarded so PRs do not push to the registry.
- `cache-from` has no bare image tags.
- `docker/setup-buildx-action` runs before the build.

## Verification

- Confirm no cache mounts remain: `grep -rn "mount=type=cache" --include=Dockerfile .`
- Validate workflow YAML: `ruby -ryaml -e 'Dir[".github/workflows/*.yml"].each{|f| YAML.load_file(f)}; puts "YAML OK"'`
- The first run after switching to the registry cache is cold; later runs should show `CACHED` for the dependency layer.
