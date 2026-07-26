PACKAGE=treemap-extractor

build:
	docker build -t $(PACKAGE):latest .

push:
	@echo $(CR_TOKEN) | docker login ghcr.io -u $(CR_USER) --password-stdin
	docker tag $(PACKAGE):latest ghcr.io/$(CR_USER)/$(PACKAGE):latest
	docker push ghcr.io/$(CR_USER)/$(PACKAGE):latest
	docker rmi ghcr.io/$(CR_USER)/$(PACKAGE):latest

format:
	@true

check:
	python3 -m flake8 app/

shell:
	docker run --rm -v $(PWD)/var:/app/var -u $(shell id -u):$(shell id -g) -it $(PACKAGE):latest bash

test-run:
	docker run --rm -v $(PWD)/var:/app/var -u $(shell id -u):$(shell id -g) -e "GPX_OFFSET=471.3" -e "MASK_SIZE=0.35" -t $(PACKAGE):latest bin/process

test-job:
	docker run --rm -v $(PWD)/var:/app/var -u $(shell id -u):$(shell id -g) -e "GPX_OFFSET=471.3" -e "MASK_SIZE=0.35" -t $(PACKAGE):latest bin/process
