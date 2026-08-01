PACKAGE=treemap-extractor

BATCH_QUEUE=treemap-transcode
BATCH_DEFINITION=treemap-extractor
DATASET_URL?=s3://treemap-panoramas-tmp-g6dxkoua/437014194731225088/
RESULT_URL?=s3://panoramas-px3qodu3/437014194731225088/

build: docker-login
	docker build -t $(PACKAGE):latest .

push: docker-login
	docker tag $(PACKAGE):latest ghcr.io/$(CR_USER)/$(PACKAGE):latest
	docker push ghcr.io/$(CR_USER)/$(PACKAGE):latest
	docker rmi ghcr.io/$(CR_USER)/$(PACKAGE):latest

docker-login:
	@echo $(CR_TOKEN) | docker login ghcr.io -u $(CR_USER) --password-stdin

format:
	@true

check:
	python3 -m flake8 app/

shell:
	docker run --rm -v $(PWD)/var:/app/var -u $(shell id -u):$(shell id -g) -e "DATASET_URL=$(DATASET_URL)" -it $(PACKAGE):latest bash

test-run:
	docker run --rm -v $(PWD)/var:/app/var -u $(shell id -u):$(shell id -g) -e "GPX_OFFSET=471.3" -e "MASK_SIZE=0.35" -e "DATASET_URL=$(DATASET_URL)" -t $(PACKAGE):latest bin/process

test-job:
	aws batch submit-job --job-name $(PACKAGE)-test-$(shell date +'%Y%m%d-%H%M%S') --job-queue $(BATCH_QUEUE) --job-definition $(BATCH_DEFINITION) --container-overrides '{ "command": ["bin/process"], "environment": [ {"name": "AWS_ACCESS_KEY_ID", "value": "$(FILES_KEY)"}, {"name": "AWS_SECRET_ACCESS_KEY", "value": "$(FILES_SECRET)"}, {"name": "AWS_REGION", "value": "$(FILES_REGION)"}, {"name": "AWS_ENDPOINT_URL", "value": "$(FILES_ENDPOINT)"}, {"name": "GPX_OFFSET", "value": "471.3" }, {"name": "MASK_SIZE", "value": "0.35"}, {"name": "DATASET_URL", "value": "$(DATASET_URL)"}, {"name": "RESULT_URL", "value": "$(RESULT_URL)"} ] }'
