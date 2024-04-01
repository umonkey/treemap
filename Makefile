VERSION=0.1.3

CR_USER=umonkey

help:
	@echo "Development steps:"
	@echo ""
	@echo "make build         -- compile backend and frontend"
	@echo "make lint          -- run linters for both backend and frontend"
	@echo "make serve         -- run the remote Docker image"
	@echo "make serve-local   -- run the local Docker image"
	@echo ""
	@echo "Release steps:"
	@echo ""
	@echo "make build-image   -- build the Docker image"
	@echo "make cr-login      -- log in to the container repository"
	@echo "make publish-image -- publish the Docker image at ghcr.io"

build:
	make -C backend build
	make -C frontend build

build-image:
	docker build --ulimit nofile=5000:5000 --tag treemap:$(VERSION) --file container/Dockerfile .

lint:
	make -C backend lint
	make -C frontend lint

# Log in to the container repository.
# Normally only needed once in a while.
#
# Get your personal GitHub token (classic) here:
# https://github.com/settings/tokens
cr-login:
ifeq (,$(CR_PAT))
	$(error CR_PAT is not set.)
else
	echo $$CR_PAT | docker login ghcr.io -u $(CR_USER) --password-stdin
endif

publish-image:
	docker tag treemap:$(VERSION) ghcr.io/$(CR_USER)/treemap:$(VERSION)
	docker tag treemap:$(VERSION) ghcr.io/$(CR_USER)/treemap:latest
	docker push ghcr.io/$(CR_USER)/treemap:$(VERSION)
	docker push ghcr.io/$(CR_USER)/treemap:latest

serve:
	docker run -it --rm -v $(PWD)/var:/app/var -p 8000:8000 ghcr.io/$(CR_USER)/treemap:$(VERSION)

serve-local:
	mkdir -p var
	docker run -it --rm -v $(PWD)/var:/app/var -p 8000:8000 treemap:$(VERSION)
