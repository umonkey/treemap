build:
	docker build -t treemap-extractor:latest .

format:
	@true

check:
	python3 -m flake8 app/

shell:
	docker run --rm -v $(PWD)/var:/app/var -u $(shell id -u):$(shell id -g) -it treemap-extractor:latest bash

test-run:
	docker run --rm -v $(PWD)/var:/app/var -u $(shell id -u):$(shell id -g) -e "GPX_OFFSET=471.3" -e "MASK_SIZE=0.35" -t treemap-extractor:latest bin/process
