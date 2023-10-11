#!/usr/bin/env sh

location=$1

if [ -n "$location" ]; then
	story_count=$(find "$location" -mindepth 1 -maxdepth 1 -type d | wc -l)
	echo "$story_count"
else
	echo 0
fi
