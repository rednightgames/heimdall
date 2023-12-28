GIT_SHA = $(shell git rev-parse --verify HEAD)
IMAGES_TAG = ${shell git describe --exact-match 2> /dev/null || echo "latest"}

IMAGE_DIRS = $(wildcard services/*)

.PHONY: all ${IMAGE_DIRS} packages

all: ${IMAGE_DIRS} packages

${IMAGE_DIRS}: packages
	$(eval IMAGE_NAME := $(word 2,$(subst /, ,$@)))
	docker build -t heimdall/${IMAGE_NAME}:${IMAGES_TAG} -t heimdall/${IMAGE_NAME}:latest --build-arg TAG=${IMAGES_TAG} -- $@

packages:
	docker build -t heimdall/base:${IMAGES_TAG} -t heimdall/base:latest --build-arg TAG=${IMAGES_TAG} --build-arg GIT_SHA=${GIT_SHA} $@
