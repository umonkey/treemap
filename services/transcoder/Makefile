PACKAGE=treemap-transcoder

BATCH_QUEUE=treemap-transcode
BATCH_DEFINITION=treemap-transcoder
INPUT_URL?=s3://treemap-panoramas-tmp-g6dxkoua/437014194731225088/video.mp4
OUTPUT_URL?=s3://treemap-panoramas-tmp-g6dxkoua/437014194731225088/video-360p.mp4

build:
	docker build -t $(PACKAGE):latest .

push:
	echo $(CR_TOKEN) | docker login ghcr.io -u $(CR_USER) --password-stdin
	docker tag $(PACKAGE):latest ghcr.io/$(CR_USER)/$(PACKAGE):latest
	docker push ghcr.io/$(CR_USER)/$(PACKAGE):latest
	docker rmi ghcr.io/$(CR_USER)/$(PACKAGE):latest

test-run:
	docker run --rm --env AWS_ACCESS_KEY_ID="$(FILES_KEY)" --env AWS_SECRET_ACCESS_KEY="$(FILES_SECRET)" --env AWS_REGION="$(FILES_REGION)" --env AWS_ENDPOINT_URL="$(FILES_ENDPOINT)" $(PACKAGE) $(INPUT_URL) $(OUTPUT_URL)

test-task:
	aws batch submit-job --job-name $(PACKAGE)-test --job-queue $(BATCH_QUEUE) --job-definition $(BATCH_DEFINITION) --container-overrides '{ "command": ["$(INPUT_URL)", "$(OUTPUT_URL)"], "environment": [ {"name": "AWS_ACCESS_KEY_ID", "value": "$(FILES_KEY)"}, {"name": "AWS_SECRET_ACCESS_KEY", "value": "$(FILES_SECRET)"}, {"name": "AWS_REGION", "value": "$(FILES_REGION)"}, {"name": "AWS_ENDPOINT_URL", "value": "$(FILES_ENDPOINT)"} ] }'
