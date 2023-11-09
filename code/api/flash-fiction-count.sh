#!/usr/bin/env sh

location=$1

if [ -n "$location" ]; then
	flash_fiction_count=$(find "$location" -type f -name "*.md" | wc -l)
	echo "$flash_fiction_count"
else
	echo 0
fi
