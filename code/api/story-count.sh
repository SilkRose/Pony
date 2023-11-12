#!/usr/bin/env sh

set -o errexit
set -o nounset

location=$1

if [ -d "$location" ]; then
	story_count=$(find "$location" -mindepth 1 -maxdepth 1 -type d | wc -l)
	echo "$story_count"
else
	echo 0
fi
