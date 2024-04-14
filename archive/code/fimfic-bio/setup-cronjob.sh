#!/usr/bin/env sh

set -o errexit
set -o nounset

if [ "$#" -ne 4 ]; then
	echo "You must provide the full path for each of these files!"
	echo "Usage: $0 <node_executable> <script_path> <recommendations JSON> <cookies JSON>"
	exit 1
fi

node=$1
mane=$2
recommendations=$3
cookies=$4

CRON_JOB="0 10 * * * $node $mane $recommendations $cookies"

(
	crontab -l
	echo "$CRON_JOB"
) | sort - | uniq - | crontab -
