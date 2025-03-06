#!/usr/bin/env bash

DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR

bun x envexpand ../../../conf/srv/r.env ../../../conf/srv/cron/_pg.env >/tmp/authCron.env
deno=$(which deno)
set -ex

env - $deno run --env-file=/tmp/authCron.env --unstable-cron -A main.js
