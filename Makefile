VERSION=0.1.3

CR_USER ?= umonkey

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
	docker compose build

image-shell:
	docker run -it --rm -w /app treemap:$(VERSION) sh

lint:
	make -C backend lint
	make -C frontend lint

serve:
	docker compose up

serve-local:
	mkdir -p var
	docker run -it --rm -v $(PWD)/var:/app/var -p 8001:8000 treemap:$(VERSION)

start:
	docker compose up
