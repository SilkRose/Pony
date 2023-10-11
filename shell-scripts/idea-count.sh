#!/usr/bin/env sh

location=$1

if [ -n "$location" ]; then
	idea_count=$(grep -c "^## " < "$location")
	echo "$idea_count"
else
	echo 0
fi
