VERSION=0.1.3

CR_USER ?= umonkey
BACKEND = docker run --rm -v $(PWD)/backend:/app -v $(PWD)/.cache/rust-registry:/usr/local/cargo/registry -w /app treemap_rust_builder:latest

help:
	@echo "Development steps:"
	@echo ""
	@echo "make build                 -- compile backend and frontend"
	@echo "make build-backend-docker  -- build the backend image"
	@echo "make lint                  -- run linters for both backend and frontend"
	@echo "make serve                 -- run the remote Docker image"
	@echo "make serve-local           -- run the local Docker image"
	@echo ""
	@echo "Release steps:"
	@echo ""
	@echo "make build-image           -- build the Docker image"
	@echo "make cr-login              -- log in to the container repository"
	@echo "make publish-image         -- publish the Docker image at ghcr.io"

build:
	make -C backend build
	make -C frontend build

rust-builder:
	docker build --tag treemap_rust_builder:latest --file container/Dockerfile.rust-builder .

# Build the backend binary using a dockerized build environment.
# This is useful for CI/CD pipelines, letting us cache the intermediate
# files between builds.  If we used a single dockerfile for building,
# we'd have to start the build process from scratch every time, which
# takes a lot of time.
build-backend-docker:
	mkdir -p .cache/rust-registry
	$(BACKEND) cargo build --release
	$(BACKEND) chown -R 1000:1000 /usr/local/cargo/registry

build-frontend-docker:
	mkdir -p .cache/npm
	docker run --rm -v $(PWD)/frontend:/app -v $(PWD)/.cache/npm:/root/.npm -w /app -e VITE_SENTRY_AUTH_TOKEN=$(SENTRY_AUTH_TOKEN) -e VITE_API_ROOT=$(API_ROOT) --network=host --ulimit nofile=5000:5000 node:20-alpine3.18 sh docker-build.sh

build-image: build-backend-docker build-frontend-docker
	docker build --tag treemap:$(VERSION) --file container/Dockerfile .

image-shell:
	docker run -it --rm -w /app treemap:$(VERSION) sh

lint:
	make -C backend lint
	make -C frontend lint

# Log in to the container repository.
# Normally only needed once in a while.
#
# Get your personal GitHub token (classic) here:
# https://github.com/settings/tokens
cr-login:
ifeq (,$(CR_TOKEN))
	$(error CR_TOKEN is not set.)
else
	echo $$CR_TOKEN | docker login ghcr.io -u $(CR_USER) --password-stdin
endif

publish-image:
	docker tag treemap:$(VERSION) ghcr.io/$(CR_USER)/treemap:$(VERSION)
	docker tag treemap:$(VERSION) ghcr.io/$(CR_USER)/treemap:latest
	docker push ghcr.io/$(CR_USER)/treemap:$(VERSION)
	docker push ghcr.io/$(CR_USER)/treemap:latest

serve:
	docker run -it --rm -v $(PWD)/var:/app/var -p 8001:8000 ghcr.io/$(CR_USER)/treemap:$(VERSION)

serve-local:
	mkdir -p var
	docker run -it --rm -v $(PWD)/var:/app/var -p 8001:8000 treemap:$(VERSION)

test-backend:
	mkdir -p .cache/rust-registry
	$(BACKEND) cargo clippy
	$(BACKEND) cargo test
