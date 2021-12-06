#!/bin/bash

if [ $# -ne 1 ]; then
  echo "指定された引数は$#個です。" 1>&2
  echo "実行するには1個の引数が必要です。" 1>&2
  exit 1
fi

sudo systemctl stop run-idea
sudo systemctl disable run-idea

find . -type f -print0 | xargs -0 sudo chown root:root
sudo mv run-idea.service /etc/systemd/system
find . -type f -print0 | xargs -0 sudo chmod +x
find . -type f -print0 | xargs -0 --replace=res_files sudo mv -f res_files $1

sudo systemctl enable run-idea
sudo systemctl restart run-idea
