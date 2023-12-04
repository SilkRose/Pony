#!/usr/bin/env sh

set -o errexit
set -o nounset

location=$1

if [ -d "$location" ]; then
	flash_fiction_count=$(find "$location" -type f -name "*.md" | wc -l)
	echo "$flash_fiction_count"
else
	echo 0
fi
