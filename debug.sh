#!/bin/bash
set -a; eval "$(cat .env <(echo) <(declare -x))"; set +a;
cargo run -p c-presentation -- -v
