#!/bin/bash
IMAGE=ghcr.io/lucky3028/idea-discussion-master

docker-compose down
# https://stackoverflow.com/questions/49316462/how-to-update-existing-images-with-docker-compose
docker rmi -f $(docker images | grep "$IMAGE" | awk '{print$3}')
docker pull ${IMAGE}:latest
docker-compose up -d --force-recreate
