GIT_SHA = $(shell git rev-parse --verify HEAD)
IMAGES_TAG = ${shell git describe --exact-match 2> /dev/null || echo "latest"}

IMAGE_DIRS = $(wildcard services/*)

.PHONY: all ${IMAGE_DIRS} proto packages lint

all: ${IMAGE_DIRS} proto packages

${IMAGE_DIRS}: proto packages
	$(eval IMAGE_NAME := $(word 2,$(subst /, ,$@)))
	docker build -t heimdall/${IMAGE_NAME}:${IMAGES_TAG} -t heimdall/${IMAGE_NAME}:latest --build-arg TAG=${IMAGES_TAG} -- $@

proto:
	docker build -t heimdall/proto:${IMAGES_TAG} -t heimdall/proto:latest --build-arg TAG=${IMAGES_TAG} --build-arg GIT_SHA=${GIT_SHA} $@

packages:
	docker build -t heimdall/base:${IMAGES_TAG} -t heimdall/base:latest --build-arg TAG=${IMAGES_TAG} --build-arg GIT_SHA=${GIT_SHA} $@

gateway-watch:
	cargo watch -x "run --bin gateway" -w ./services/gateway/

config-watch:
	cargo watch -x "run --bin config" -w ./services/config/

lint:
	cargo fmt --all -- --check

lint-fix:
	cargo fmt --all --
