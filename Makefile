CR_USER = umonkey
CHANGED_SERVICES = $(shell git status --porcelain | grep "^.. services/" | cut -c 4- | cut -d/ -f1,2 | sort -u | tr '\n' ' ')

# Docker Compose configuration for isolated local development.
# The existing development and production commands remain unchanged.
COMPOSE_LOCAL = docker compose -f compose.yaml -f compose.local.yaml

help:
	@echo "Development steps:"
	@echo ""
	@echo "make build                 -- compile backend and frontend"
	@echo "make start                 -- run the development version"
	@echo "make sql                   -- run sqlite3 in backend container"
	@echo "make start-local           -- start isolated local development"
	@echo "make stop-local            -- stop local development"
	@echo "make logs-local            -- show local container logs"
	@echo "make sql-local             -- open the local SQLite database"

build:
	docker compose build

build-combined:
	docker build -t treemap:latest -f Dockerfile .

check:
	for service in $(CHANGED_SERVICES); do make -C $$service check; done

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

.PHONY: local-init start-local stop-local logs-local sql-local

local-init:
	sh tools/init-local.sh

start-local: local-init
	$(COMPOSE_LOCAL) up --build

stop-local:
	$(COMPOSE_LOCAL) down

logs-local:
	$(COMPOSE_LOCAL) logs -f

sql-local:
	$(COMPOSE_LOCAL) exec backend sqlite3 /app/var/database.sqlite