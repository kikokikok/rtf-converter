#!/usr/bin/env bash
set -e

finch build --platform=$PLATFORMS --file $BUILD_CONTEXT -t $IMAGE --progress=plain .

if $PUSH_IMAGE; then
    docker push $IMAGE
fi