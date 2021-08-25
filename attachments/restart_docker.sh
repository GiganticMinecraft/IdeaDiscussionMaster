#!/bin/bash
IMAGE=ghcr.io/lucky3028/idea-discussion-master

docker-compose down
# https://stackoverflow.com/questions/49316462/how-to-update-existing-images-with-docker-compose
docker rmi -f $(docker images | grep "$IMAGE" | awk '{print$3}')
docker pull ${IMAGE}:latest
# `--force-recreate`を指定しないと、前回まで使用していたイメージが存在しないというエラーが出て、続行するかどうかを選択しなければいけなくなる
docker-compose up -d --force-recreate
