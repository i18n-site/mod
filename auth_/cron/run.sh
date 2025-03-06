#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
../../../srv/sh/cronEnv.sh
set -ex

env - $(which deno) run --env-file=/tmp/srv_cron.env --unstable-cron -A main.js
