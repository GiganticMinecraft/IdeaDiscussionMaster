#!/bin/bash
set -a; eval "$(cat .env <(echo) <(declare -x))"; set +a;
./idea-discussion-master
