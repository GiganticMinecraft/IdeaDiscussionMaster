#!/bin/bash

docker compose down
docker compose pull
docker compose up -d
# Remove untagged images
# see: https://stackoverflow.com/questions/33913020/docker-remove-none-tag-images
docker rmi $(docker images --filter "dangling=true" -q --no-trunc) || :
