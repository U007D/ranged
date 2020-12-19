#!/usr/bin/env bash

REV="$1"; shift;
IMAGE="rustc:${REV}"

docker build . --build-arg old_rev="$REV" -t "$IMAGE"
docker run -v "$PWD":/tmp --rm -t "$IMAGE" cargo $@
