#!/bin/sh

DOCKER_DEFAULT_PLATFORM=linux/amd64 docker build -t mist -f deploy/Dockerfile .
fly deploy --config demo/fly.mist.toml --image mist --local-only

DOCKER_DEFAULT_PLATFORM=linux/amd64 docker build -t mist-demo -f demo/Dockerfile demo
fly deploy --config demo/fly.demo.toml --image mist-demo --local-only

echo "😅 Done!"
