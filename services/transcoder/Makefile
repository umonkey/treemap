PACKAGE=treemap-transcoder

build:
	docker build -t $(PACKAGE):latest .

push:
	echo $(CR_TOKEN) | docker login ghcr.io -u $(CR_USER) --password-stdin
	docker tag $(PACKAGE):latest ghcr.io/$(CR_USER)/$(PACKAGE):latest
	docker push ghcr.io/$(CR_USER)/$(PACKAGE):latest
	docker rmi ghcr.io/$(CR_USER)/$(PACKAGE):latest

test-run:
	docker run --rm --env AWS_ACCESS_KEY_ID="$(FILES_KEY)" --env AWS_SECRET_ACCESS_KEY="$(FILES_SECRET)" --env AWS_REGION="$(FILES_REGION)" --env AWS_ENDPOINT_URL="$(FILES_ENDPOINT)" $(PACKAGE) s3://treemap-panoramas-tmp-g6dxkoua/437014194731225088/video.mp4 s3://treemap-panoramas-tmp-g6dxkoua/437014194731225088/video-360p.mp4
