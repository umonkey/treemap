CR_USER = umonkey
CHANGED_SERVICES = $(shell git status --porcelain | grep "^.. services/" | cut -c 4- | cut -d/ -f1,2 | sort -u | tr '\n' ' ')
CTAGS ?= ctags
TAG_DIRS = services/backend services/chatbot

.PHONY: tags

help:
	@echo "Development steps:"
	@echo ""
	@echo "make build                 -- compile backend and frontend"
	@echo "make tags                  -- generate ctags index for Rust symbols"
	@echo "make start                 -- run the development version"
	@echo "make sql                   -- run sqlite3 in backend container"

build:
	docker compose build

build-combined:
	docker build -t treemap:latest -f Dockerfile .

check:
	for service in $(CHANGED_SERVICES); do make -C $$service check; done

tags:
	@command -v $(CTAGS) >/dev/null 2>&1 || { \
		echo "error: '$(CTAGS)' not found."; \
		echo "Install Universal Ctags (Rust support is required):"; \
		echo "  Debian/Ubuntu: sudo apt install universal-ctags"; \
		echo "  macOS:         brew install universal-ctags"; \
		exit 1; \
	}
	@$(CTAGS) --list-languages 2>/dev/null | grep -qx Rust || { \
		echo "error: '$(CTAGS)' does not support Rust."; \
		echo "Install Universal Ctags; Exuberant Ctags and BSD ctags are not sufficient."; \
		exit 1; \
	}
	@$(CTAGS) --recurse --languages=Rust --fields=+n \
		--sort=yes --exclude=target --exclude=.git \
		-f tags $(TAG_DIRS)
	@echo "Generated ./tags ($$(wc -l < tags | tr -d ' ') entries)."

format-docs:
	@echo "Formatting Markdown documents..."
	@npx -y prettier --list-different --write $(shell find ./docs ./.agents -name "*.md")

format:
	for service in $(CHANGED_SERVICES); do make -C $$service format; done

push:
	echo $(CR_TOKEN) | docker login ghcr.io -u $(CR_USER) --password-stdin
	docker tag treemap:latest ghcr.io/$(CR_USER)/treemap:latest
	docker push ghcr.io/$(CR_USER)/treemap:latest
	docker rmi ghcr.io/$(CR_USER)/treemap:latest

start:
	docker compose up

start-prod:
	docker compose -f compose.prod.yaml up

load-dev:
	vegeta attack -targets=dev/loadtest-dev.txt -rate=30 -duration=30s | vegeta report

load-prod:
	vegeta attack -targets=dev/loadtest-prod.txt -rate=30 -duration=30s | vegeta report

rebuild-backend:
	docker compose build backend
	docker compose up backend -d

sql:
	docker compose run --rm backend sqlite3 /app/var/database.sqlite
