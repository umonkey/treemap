build:
	docker build -t treemap-extractor:latest .

shell:
	docker run --rm -v $(PWD)/var:/app/var -it treemap-extractor:latest bash

test-run:
	docker run --rm -v $(PWD)/var:/app/var -e "GPX_OFFSET=471.3" -t treemap-extractor:latest bin/process
