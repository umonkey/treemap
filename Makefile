CR_USER = umonkey
CHANGED_SERVICES = $(shell git status --porcelain | grep "^.. services/" | cut -c 4- | cut -d/ -f1,2 | sort -u | tr '\n' ' ')

help:
	@echo "Development steps:"
	@echo ""
	@echo "make build                 -- compile backend and frontend"
	@echo "make start                 -- run the development version"

build:
	docker compose build

build-combined:
	docker build -t treemap:latest -f Dockerfile .

check:
	for service in $(CHANGED_SERVICES); do make -C $$service check; done

format-docs:
	npx -y prettier --write $(shell find ./docs ./.agents -name "*.md")

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
