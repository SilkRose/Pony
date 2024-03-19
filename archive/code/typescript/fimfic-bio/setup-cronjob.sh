#!/usr/bin/env sh

set -o errexit
set -o nounset

mane=$1
cookies=$2

CRON_JOB="0 0 * * * node $mane $(cat "$cookies")"

(crontab -l; echo "$CRON_JOB") | sort - | uniq - | crontab -
